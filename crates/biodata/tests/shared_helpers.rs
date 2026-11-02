//! Shared DNA, code-table, ontology, and transcript-region values through biodata's public API.
use biodata::{
    Base, CoordinateError, GeneticCode, ModeOfInheritance, Position1, Residue, Sequence,
    TranscriptPosition, TranscriptRegion, TranscriptRegionKind,
    hgvs::{Anchor, Position, PositionAnchor},
    reverse_complement, without_chr_prefix,
};
use std::num::NonZeroI32;

#[test]
fn dna_bases_parse_and_reverse_complement() {
    for (letter, complement) in [
        ('A', 'T'),
        ('C', 'G'),
        ('G', 'C'),
        ('T', 'A'),
        ('R', 'Y'),
        ('Y', 'R'),
        ('S', 'S'),
        ('W', 'W'),
        ('K', 'M'),
        ('M', 'K'),
        ('B', 'V'),
        ('D', 'H'),
        ('H', 'D'),
        ('V', 'B'),
        ('N', 'N'),
    ] {
        let base = Base::from_letter(letter).unwrap();
        assert_eq!(base.complement(), Base::from_letter(complement).unwrap());
        assert_eq!(base.complement().complement(), base);
    }
    assert_eq!(Base::from_letter('U'), None);
    assert_eq!("U".parse::<Sequence>(), Err(CoordinateError::RnaSequence));
    assert_eq!(Base::from_letter('a'), None);
    let dna = [Base::A, Base::R, Base::C, Base::N];
    assert_eq!(
        reverse_complement(&dna),
        [Base::N, Base::G, Base::Y, Base::T]
    );
}

#[test]
fn ncbi_translation_tables_keep_their_distinct_codons() {
    let standard = GeneticCode::from_ncbi_table(1).unwrap();
    let mitochondrial = GeneticCode::from_ncbi_table(2).unwrap();
    assert_eq!(standard.ncbi_table(), 1);
    assert_eq!(mitochondrial.ncbi_table(), 2);
    assert_eq!(GeneticCode::from_ncbi_table(3), None);
    for (codon, expected_standard, expected_mitochondrial) in [
        (b"ATA".as_slice(), Residue::Ile, Residue::Met),
        (b"TGA".as_slice(), Residue::Ter, Residue::Trp),
        (b"AGA".as_slice(), Residue::Arg, Residue::Ter),
        (b"AGG".as_slice(), Residue::Arg, Residue::Ter),
        (b"ATG".as_slice(), Residue::Met, Residue::Met),
    ] {
        assert_eq!(standard.exact_residue(codon), expected_standard);
        assert_eq!(mitochondrial.exact_residue(codon), expected_mitochondrial);
    }
    assert_eq!(standard.exact_residue(b"NNN"), Residue::Xaa);
    assert_eq!(standard.residue(b"ATH"), Residue::Ile);
    assert_eq!(mitochondrial.residue(b"ATH"), Residue::Xaa);
    assert_eq!(standard.residue(b"TGR"), Residue::Xaa);
    assert_eq!(mitochondrial.residue(b"TGR"), Residue::Trp);
    // Each IUPAC possibility set keeps the table's one residue when all
    // represented exact codons agree.
    for (codon, expected) in [
        (b"TGY", Residue::Cys),
        (b"GCS", Residue::Ala),
        (b"TCW", Residue::Ser),
        (b"GGK", Residue::Gly),
        (b"CCM", Residue::Pro),
        (b"GGB", Residue::Gly),
        (b"GCD", Residue::Ala),
        (b"ACV", Residue::Thr),
    ] {
        assert_eq!(standard.residue(codon), expected, "{codon:?}");
    }
}

#[test]
fn hpo_inheritance_codes_round_trip() {
    for term in [
        ModeOfInheritance::AUTOSOMAL_DOMINANT,
        ModeOfInheritance::AUTOSOMAL_RECESSIVE,
        ModeOfInheritance::MITOCHONDRIAL,
        ModeOfInheritance::SEMIDOMINANT,
        ModeOfInheritance::X_LINKED,
        ModeOfInheritance::MODE_OF_INHERITANCE,
    ] {
        assert_eq!(
            ModeOfInheritance::from_hpo_code(term.hpo_code()),
            Some(term)
        );
    }
    assert_eq!(ModeOfInheritance::from_hpo_code("HP:0000000"), None);
}

#[test]
fn transcript_regions_validate_numbered_and_flanking_positions() {
    use TranscriptRegionKind::{Downstream, Exon, Intron, Upstream};
    let exon = TranscriptRegion::numbered(Exon, 2, 3).unwrap();
    assert_eq!(
        (exon.kind(), exon.number(), exon.total()),
        (Exon, Some(2), Some(3))
    );
    assert_eq!(TranscriptRegion::numbered(Intron, 0, 3), None);
    assert_eq!(TranscriptRegion::numbered(Intron, 3, 2), None);
    assert_eq!(TranscriptRegion::numbered(Upstream, 1, 1), None);
    let flank = TranscriptRegion::flank(Downstream).unwrap();
    assert_eq!((flank.number(), flank.total()), (None, None));
    assert_eq!(TranscriptRegion::flank(Exon), None);
}

#[test]
fn transcript_positions_keep_known_anchor_and_signed_nonzero_offset() {
    let one = Position1::new(1).unwrap();
    for anchor in [
        Anchor::Upstream { distance: one },
        Anchor::Coding { position: one },
        Anchor::Downstream { distance: one },
        Anchor::Noncoding { position: one },
    ] {
        let plain = TranscriptPosition::new(anchor, None);
        assert_eq!(plain.anchor(), anchor);
        assert_eq!(plain.offset(), None);
        let grammar: Position = plain.into();
        assert_eq!(grammar.anchor, PositionAnchor::Transcript(anchor));
        assert_eq!(grammar.offset, None);
        for signed in [-4, 5] {
            let offset = NonZeroI32::new(signed).unwrap();
            let intronic = TranscriptPosition::new(anchor, Some(offset));
            assert_eq!(intronic.anchor(), anchor);
            assert_eq!(intronic.offset(), Some(offset));
        }
    }
    assert_eq!(NonZeroI32::new(0), None);
}

#[test]
fn source_chromosome_label_keeps_the_written_suffix() {
    assert_eq!(without_chr_prefix("chrM"), "M");
    assert_eq!(without_chr_prefix("chr7"), "7");
    assert_eq!(without_chr_prefix("chrUn"), "Un");
    assert_eq!(without_chr_prefix("7"), "7");
}
