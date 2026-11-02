//! Source-preserved variant expressions with their declared syntax.
use crate::{Accession, Contig, HgvsText, Position1, Sequence, Text};

/// Published syntax label for a source-preserved expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExpressionSyntax {
    /// HGVS genomic description.
    #[cfg_attr(feature = "serde", serde(rename = "hgvs.g"))]
    HgvsG,
    /// HGVS coding description.
    #[cfg_attr(feature = "serde", serde(rename = "hgvs.c"))]
    HgvsC,
    /// HGVS noncoding transcript description.
    #[cfg_attr(feature = "serde", serde(rename = "hgvs.n"))]
    HgvsN,
    /// HGVS RNA description.
    #[cfg_attr(feature = "serde", serde(rename = "hgvs.r"))]
    HgvsR,
    /// HGVS protein description.
    #[cfg_attr(feature = "serde", serde(rename = "hgvs.p"))]
    HgvsP,
    /// HGVS mitochondrial description.
    #[cfg_attr(feature = "serde", serde(rename = "hgvs.m"))]
    HgvsM,
    /// HGVS circular genomic description.
    #[cfg_attr(feature = "serde", serde(rename = "hgvs.o"))]
    HgvsO,
    /// NCBI SPDI interbase description.
    #[cfg_attr(feature = "serde", serde(rename = "spdi"))]
    Spdi,
    /// gnomAD chromosome-position-reference-alternate identifier.
    #[cfg_attr(feature = "serde", serde(rename = "gnomad"))]
    Gnomad,
    /// Padded VCF chromosome-position-reference-alternate text.
    #[cfg_attr(feature = "serde", serde(rename = "vcf"))]
    Vcf,
    /// Source-preserved cytogenetic notation.
    #[cfg_attr(feature = "serde", serde(rename = "iscn"))]
    Iscn,
}

/// A source expression does not match its declared syntax.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExpressionError;
impl ExpressionError {
    /// Shared refusal code.
    pub const fn code(self) -> &'static str {
        "notation.invalid"
    }
}
impl std::fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for ExpressionError {}

/// Source expression checked for its declared published form.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Fields"))]
pub struct Expression(Fields);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Fields {
    syntax: ExpressionSyntax,
    value: Text,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    syntax_version: Option<Text>,
}

impl Expression {
    /// Retain source text after a form check.
    pub fn new(
        syntax: ExpressionSyntax,
        value: Text,
        syntax_version: Option<Text>,
    ) -> Result<Self, ExpressionError> {
        if matches!(
            syntax,
            ExpressionSyntax::Spdi | ExpressionSyntax::Gnomad | ExpressionSyntax::Vcf
        ) {
            if !literal_form(syntax, value.as_str()) {
                return Err(ExpressionError);
            }
            return Ok(Self(Fields {
                syntax,
                value,
                syntax_version,
            }));
        }
        let expected = match syntax {
            ExpressionSyntax::HgvsG => 'g',
            ExpressionSyntax::HgvsC => 'c',
            ExpressionSyntax::HgvsN => 'n',
            ExpressionSyntax::HgvsR => 'r',
            ExpressionSyntax::HgvsP => 'p',
            ExpressionSyntax::HgvsM => 'm',
            ExpressionSyntax::HgvsO => 'o',
            _ => return Err(ExpressionError),
        };
        let (_, body) = value.as_str().split_once(':').ok_or(ExpressionError)?;
        let qualified = value.as_str().parse::<HgvsText>().is_ok();
        let chromosome = syntax == ExpressionSyntax::HgvsG
            && value
                .as_str()
                .strip_prefix("chr")
                .and_then(|value| value.split_once(':'))
                .is_some_and(|(name, _)| name == "M" || name.parse::<crate::Chromosome>().is_ok());
        crate::source_text::check_hgvs_body(body).map_err(|_| ExpressionError)?;
        if (!qualified && !chromosome)
            || !body.starts_with(&format!("{expected}."))
            || body.len() < 3
        {
            return Err(ExpressionError);
        }
        Ok(Self(Fields {
            syntax,
            value,
            syntax_version,
        }))
    }

    /// Unchanged source notation.
    pub fn value(&self) -> &Text {
        &self.0.value
    }

    /// Source-stated syntax version, when present.
    pub fn syntax_version(&self) -> Option<&Text> {
        self.0.syntax_version.as_ref()
    }

    /// Declared notation syntax.
    pub fn syntax(&self) -> ExpressionSyntax {
        self.0.syntax
    }

    /// Construct from reference-qualified HGVS without interpreting its edit.
    pub fn hgvs(value: Text) -> Result<Self, ExpressionError> {
        let syntax = match value
            .as_str()
            .split_once(':')
            .and_then(|(_, body)| body.chars().next())
        {
            Some('g') => ExpressionSyntax::HgvsG,
            Some('c') => ExpressionSyntax::HgvsC,
            Some('n') => ExpressionSyntax::HgvsN,
            Some('r') => ExpressionSyntax::HgvsR,
            Some('p') => ExpressionSyntax::HgvsP,
            Some('m') => ExpressionSyntax::HgvsM,
            Some('o') => ExpressionSyntax::HgvsO,
            _ => return Err(ExpressionError),
        };
        Self::new(syntax, value, None)
    }
}

#[cfg(feature = "serde")]
impl TryFrom<Fields> for Expression {
    type Error = ExpressionError;
    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        Self::new(fields.syntax, fields.value, fields.syntax_version)
    }
}

fn literal_form(syntax: ExpressionSyntax, value: &str) -> bool {
    let parts: Vec<_> = value
        .split(if syntax == ExpressionSyntax::Gnomad {
            '-'
        } else {
            ':'
        })
        .collect();
    let [reference, position, deleted, inserted] = parts.as_slice() else {
        return false;
    };
    if syntax == ExpressionSyntax::Spdi {
        reference.parse::<Accession>().is_ok()
            && position.parse::<u32>().is_ok()
            && (deleted.parse::<Sequence>().is_ok() || deleted.parse::<u32>().is_ok())
            && inserted.parse::<Sequence>().is_ok()
    } else {
        Contig::new(reference).is_ok()
            && position
                .parse::<u32>()
                .ok()
                .is_some_and(|position| Position1::new(position).is_ok())
            && !deleted.is_empty()
            && !inserted.is_empty()
            && deleted.parse::<Sequence>().is_ok()
            && inserted.parse::<Sequence>().is_ok()
    }
}

#[cfg(feature = "serde")]
crate::serde_object::object_deserialize!(Expression, Fields);
