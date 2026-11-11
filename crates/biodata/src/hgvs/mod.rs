//! HGVS variant descriptions: an optional reference, then a change.

pub(crate) mod nucleotide;
pub(crate) mod protein;

mod nucleotide_value;
pub use nucleotide::{
    Anchor, Bound, Location, Molecule, NaBody, NaChange, NaCode, NaEdit, Position, PositionAnchor,
    Whole,
};
pub use nucleotide_value::{Nucleotide, Nucleotides, RnaBase};
pub use protein::{
    Distance, Extension, ProteinChange, ProteinCode, ProteinRange, ProteinRangeError, ProteinSite,
};

use crate::outcome::{Code, Parsed};
use crate::parse::{Expect, Input, Result, run, step};
use crate::value::{Accession, GeneSymbol, accession, gene_symbol_prefix};
use std::fmt;
use winnow::Parser;
use winnow::combinator::{alt, cut_err, delimited, opt};

/// Parses one HGVS description.
pub fn parse(text: &str) -> Parsed<Hgvs> {
    run(text, hgvs)
}

/// One HGVS description.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct Hgvs {
    /// Whether the change is predicted.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "std::ops::Not::not"))]
    pub predicted: bool,
    /// The reference sequence, when written.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub reference: Option<Reference>,
    /// The change.
    pub change: Change,
}

/// A reference sequence, with an optional inner accession or gene in parentheses.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct Reference {
    /// The reference sequence.
    pub accession: Accession,
    /// What the parentheses name.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub inner: Option<Inner>,
}

/// What a reference's parentheses name. Text with an accession's prefix is an
/// accession, and anything else is a gene.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Inner {
    /// An accession, such as a transcript on a genomic reference.
    Accession(Accession),
    /// A gene symbol.
    Gene(GeneSymbol),
}

/// The change a description states.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Change {
    /// A change to a nucleotide sequence.
    Nucleotide(NaChange),
    /// A change to a protein.
    Protein(ProteinChange),
}

crate::table::codes! {
/// Refusals of the reference and molecule.
pub enum HgvsCode {
    /// The reference or its colon is malformed.
    ReferenceInvalid => "hgvs.reference.invalid",
    /// No molecule such as `g.` starts the change.
    MoleculeInvalid => "hgvs.molecule.invalid",
}
}

const REFERENCE: Expect = Expect {
    code: Code::Hgvs(HgvsCode::ReferenceInvalid),
    label: "reference",
};
const MOLECULE: Expect = Expect {
    code: Code::Hgvs(HgvsCode::MoleculeInvalid),
    label: "molecule",
};

fn hgvs(input: &mut Input<'_>) -> Result<Hgvs> {
    step(input)?;
    let reference = opt(reference.context(REFERENCE)).parse_next(input)?;
    let (change, predicted) = cut_err(alt((
        nucleotide::change.map(|(change, predicted)| (Change::Nucleotide(change), predicted)),
        predicted_protein.map(|change| (change, true)),
        ("p.", cut_err(protein::body).context(protein::EDIT))
            .map(|(_, change)| (Change::Protein(change), false)),
    )))
    .context(MOLECULE)
    .parse_next(input)?;
    Ok(Hgvs {
        reference,
        predicted,
        change,
    })
}

fn predicted_protein(input: &mut Input<'_>) -> Result<Change> {
    "p.(".parse_next(input)?;
    // The body parser shares the same located cursor and budget.
    let value = cut_err(protein::body).parse_next(input)?;
    cut_err(')').parse_next(input)?;
    Ok(Change::Protein(value))
}

/// An accession commits to a reference: any parentheses, then a colon.
fn reference(input: &mut Input<'_>) -> Result<Reference> {
    step(input)?;
    let inner = alt((
        hgvs_accession.map(Inner::Accession),
        (|input: &mut Input<'_>| gene_symbol_prefix(input, input.len())).map(Inner::Gene),
    ));
    let inner = opt(delimited('(', cut_err(inner), cut_err(')')));
    let accession = hgvs_accession(input)?;
    let (inner, _) = cut_err((inner, ':')).parse_next(input)?;
    Ok(Reference { accession, inner })
}

fn hgvs_accession(input: &mut Input<'_>) -> Result<Accession> {
    accession(input).map_err(|error| error.map(|failure| failure.override_context(REFERENCE)))
}

impl fmt::Display for Hgvs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(reference) = &self.reference {
            write!(f, "{reference}:")?;
        }
        match &self.change {
            Change::Nucleotide(change) => {
                change.render(f, if self.predicted { ("(", ")") } else { ("", "") })
            }
            Change::Protein(change) => {
                if self.predicted {
                    write!(f, "p.({change})")
                } else {
                    write!(f, "p.{change}")
                }
            }
        }
    }
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nucleotide(change) => write!(f, "{change}"),
            Self::Protein(change) => fmt::Display::fmt(change, f),
        }
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.accession)?;
        match &self.inner {
            Some(Inner::Accession(inner)) => write!(f, "({inner})"),
            Some(Inner::Gene(gene)) => write!(f, "({gene})"),
            None => Ok(()),
        }
    }
}
