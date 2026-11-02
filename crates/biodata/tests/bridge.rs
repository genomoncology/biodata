//! A caller reads stated matching dimensions through the public Rust API.
#![allow(clippy::panic, reason = "a missing caller dimension fails the test")]

use biodata::alteration::{
    self, Alteration, Category, Change, Comparison, Cytogenetic, ExpressionState, Kind, Scope,
    ScoreBound, ScoringSystem, Somy,
};
use biodata::hgvs::{Anchor, Location, Molecule, NaBody, NaEdit, PositionAnchor, ProteinChange};
use biodata::iscn::Chromosome;
use biodata::{Base, GeneSymbol, Outcome, Residue};

fn parsed(text: &str) -> Alteration {
    match alteration::parse(text).outcome {
        Outcome::Parsed(value) => value,
        other => panic!("{text}: expected a typed alteration, got {other:?}"),
    }
}

fn gene(text: &str) -> (GeneSymbol, Change) {
    match parsed(text).kind {
        Kind::Gene { subject, change } => (subject, change),
        other => panic!("{text}: expected a gene subject, got {other:?}"),
    }
}

#[test]
fn protein_rule_exposes_subject_site_and_replacement() {
    let (subject, change) = gene("BRAF V600E");
    assert_eq!(subject.to_string(), "BRAF");
    let Change::Protein { protein, .. } = change else {
        panic!("expected a protein change");
    };
    let ProteinChange::Substitution { site, to } = protein else {
        panic!("expected a substitution");
    };
    assert_eq!(site.position, biodata::Position1::new(600).unwrap());
    assert_eq!(site.residue, Residue::Val);
    assert_eq!(to, Residue::Glu);
}

#[test]
fn exon_rule_exposes_exon_and_stated_category() {
    let (subject, change) = gene("EGFR Exon 19 Deletion");
    assert_eq!(subject.to_string(), "EGFR");
    let Change::Event(event) = change else {
        panic!("expected an event");
    };
    let Some(Scope::Exons { first, last }) = event.scope else {
        panic!("expected an exon scope");
    };
    assert_eq!(first, 19);
    assert_eq!(last, None);
    assert_eq!(event.category, Some(Category::Deletion));
}

#[test]
fn codon_rule_exposes_codon_and_stated_category() {
    let (subject, change) = gene("KRAS Codon 12 Missense");
    assert_eq!(subject.to_string(), "KRAS");
    let Change::Event(event) = change else {
        panic!("expected an event");
    };
    assert_eq!(event.scope, Some(Scope::Codon(12)));
    assert_eq!(event.category, Some(Category::Missense));
}

#[test]
fn fusion_rule_keeps_the_whole_subject_and_stated_category() {
    let (subject, change) = gene("ALK-EML4 Fusion");
    // A caller can index the whole subject as one fusion key.
    assert_eq!(subject.to_string(), "ALK-EML4");
    let Change::Event(event) = change else {
        panic!("expected an event");
    };
    assert_eq!(event.category, Some(Category::Fusion));
}

#[test]
fn coding_rule_exposes_coordinate_system_position_and_base_edit() {
    let (subject, change) = gene("TERT c.-146C>T");
    assert_eq!(subject.to_string(), "TERT");
    let Change::Coding(coding) = change else {
        panic!("expected a nucleotide change");
    };
    assert_eq!(coding.molecule, Molecule::Coding);
    let NaBody::Edit { location, edit } = coding.body else {
        panic!("expected an edit");
    };
    let Location::Point(position) = location else {
        panic!("expected a single nucleotide position");
    };
    assert_eq!(
        position.anchor,
        PositionAnchor::Transcript(Anchor::Upstream {
            distance: biodata::Position1::new(146).unwrap()
        })
    );
    assert_eq!(position.offset, None);
    let NaEdit::Substitution { from, to } = edit else {
        panic!("expected a substitution");
    };
    assert_eq!(from, Base::C.into());
    assert_eq!(to, Base::T.into());
}

#[test]
fn nucleotide_anchor_types_preserve_molecule_distinctions_and_text() {
    use biodata::hgvs::{Change as HgvsChange, parse as parse_hgvs};
    let anchor = |text: &str| {
        let Outcome::Parsed(hgvs) = parse_hgvs(text).outcome else {
            panic!("expected parsed HGVS: {text}");
        };
        assert_eq!(hgvs.to_string(), text);
        let HgvsChange::Nucleotide(change) = hgvs.change else {
            panic!("expected nucleotide change: {text}");
        };
        let NaBody::Edit {
            location: Location::Point(position),
            ..
        } = change.body
        else {
            panic!("expected point edit: {text}");
        };
        position.anchor
    };
    let one = biodata::Position1::new(1).unwrap();
    assert_eq!(
        anchor("c.-1A>T"),
        PositionAnchor::Transcript(Anchor::Upstream { distance: one })
    );
    assert_eq!(
        anchor("c.1A>T"),
        PositionAnchor::Transcript(Anchor::Coding { position: one })
    );
    assert_eq!(
        anchor("c.*1A>T"),
        PositionAnchor::Transcript(Anchor::Downstream { distance: one })
    );
    assert_eq!(
        anchor("n.1A>T"),
        PositionAnchor::Transcript(Anchor::Noncoding { position: one })
    );
    assert_eq!(anchor("g.1A>T"), PositionAnchor::Absolute(one));
    assert_eq!(anchor("n.-1A>T"), PositionAnchor::Upstream(one));
    assert_eq!(anchor("r.1a>u"), PositionAnchor::Rna(one));
    assert_eq!(anchor("r.-1a>u"), PositionAnchor::RnaUpstream(one));
    assert_eq!(anchor("r.*1a>u"), PositionAnchor::RnaDownstream(one));
}

#[test]
fn rna_number_to_stop_boundary_insertion_remains_accepted() {
    use biodata::hgvs::parse as parse_hgvs;
    let text = "r.17_*1insu";
    let Outcome::Parsed(hgvs) = parse_hgvs(text).outcome else {
        panic!("expected parsed RNA insertion");
    };
    assert_eq!(hgvs.to_string(), text);
}

#[test]
fn expression_rule_exposes_system_comparison_exact_value_and_unit() {
    let (subject, change) = gene("PD-L1 Expression (TPS >= 50%)");
    assert_eq!(subject.to_string(), "PD-L1");
    let Change::Expression { state, score } = change else {
        panic!("expected expression");
    };
    assert_eq!(state, ExpressionState::Expression);
    let Some(score) = score else {
        panic!("expected a stated score");
    };
    assert_eq!(score.system, ScoringSystem::Tps);
    assert!(score.percent);
    let ScoreBound::Compare(comparison, value) = score.bound else {
        panic!("expected a score comparison");
    };
    assert_eq!(comparison, Comparison::GreaterOrEqual);
    assert_eq!(value.digits, 50);
    assert_eq!(value.scale, 0);
}

#[test]
fn star_rule_exposes_both_stated_alleles() {
    let (subject, change) = gene("UGT1A1 *1/*28");
    assert_eq!(subject.to_string(), "UGT1A1");
    let Change::StarAlleles { first, second } = change else {
        panic!("expected star alleles");
    };
    assert_eq!(first.number, 1);
    let Some(second) = second else {
        panic!("expected a second allele");
    };
    assert_eq!(second.number, 28);
}

#[test]
fn chromosome_rule_exposes_somy_and_chromosome() {
    let Kind::Cytogenetic(Cytogenetic::Somy { somy, chromosome }) = parsed("Monosomy 15").kind
    else {
        panic!("expected a chromosome somy");
    };
    assert_eq!(somy, Somy::Monosomy);
    assert_eq!(chromosome, Chromosome::Autosome(15));
}
