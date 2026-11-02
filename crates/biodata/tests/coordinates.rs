//! Checked coordinate construction through the public API.
use biodata::{
    AlleleKey, Base, Build, Chromosome, CoordinateError, GenomicInterval, Position1, Span0,
};

#[test]
fn positions_and_spans_refuse_invalid_arithmetic() {
    assert_eq!(Position1::new(0), Err(CoordinateError::ZeroPosition));
    let first = Position1::new(1).unwrap();
    assert_eq!(first.offset0(), 0);
    assert_eq!(first.checked_sub(1), Err(CoordinateError::ZeroPosition));
    assert!(first.checked_sub(2).is_err());
    assert_eq!(first.checked_add(1).unwrap().get(), 2);
    assert!(Position1::new(u32::MAX).unwrap().checked_add(1).is_err());
    assert_eq!("1".parse::<Position1>().unwrap().to_string(), "1");
    assert!("no".parse::<Position1>().is_err());
    for (start, end, valid) in [
        (0, 0, true),
        (2, 1, false),
        (0, u32::MAX, true),
        (u32::MAX, u32::MAX, true),
    ] {
        assert_eq!(Span0::new(start, end).is_ok(), valid);
    }
    assert!(Span0::from_offset_count(u32::MAX, 1).is_err());
    assert!(Span0::from_offset_count(0, usize::MAX).is_err());
    let outer = Span0::from_start_count(first, 4).unwrap();
    assert_eq!(outer.len(), 4);
    for (start, end, valid) in [(0, 4, true), (4, 4, true), (3, 5, false)] {
        let span = Span0::new(start, end).unwrap();
        assert_eq!(outer.contains(span), valid);
        assert_eq!(span.relative_to(outer).is_ok(), valid);
        assert_eq!(span.check_bounds(4).is_ok(), valid);
    }
    assert!(
        Span0::new(1, 2)
            .unwrap()
            .relative_to(Span0::new(2, 4).unwrap())
            .is_err()
    );
    assert!(Span0::new(u32::MAX, u32::MAX).unwrap().is_empty());
}

#[test]
fn intervals_require_the_same_coordinate_space() {
    let interval = |build, chromosome, start, end| {
        GenomicInterval::new(
            build,
            Chromosome::new(chromosome).unwrap(),
            Span0::new(start, end).unwrap(),
        )
    };
    let outer = interval(Build::GRCh38, 1, 2, 8);
    assert_eq!(
        outer.inclusive_bounds().unwrap(),
        (Position1::new(3).unwrap(), Position1::new(8).unwrap())
    );
    assert_eq!(outer.build().unwrap(), Build::GRCh38);
    assert_eq!(outer.chromosome().number(), 1);
    assert_eq!(outer.span().start(), 2);
    assert_eq!(outer.span().end(), 8);
    assert!(outer.contains(interval(Build::GRCh38, 1, 3, 4)).unwrap());
    assert!(!outer.contains(interval(Build::GRCh38, 1, 0, 4)).unwrap());
    assert_eq!(
        outer.contains(interval(Build::GRCh37, 1, 3, 4)),
        Err(CoordinateError::BuildMismatch)
    );
    assert_eq!(
        outer.contains(interval(Build::GRCh38, 2, 3, 4)),
        Err(CoordinateError::ChromosomeMismatch)
    );
    let last = Position1::new(u32::MAX).unwrap();
    assert_eq!(
        interval(Build::GRCh38, 1, u32::MAX - 1, u32::MAX).inclusive_bounds(),
        Some((last, last))
    );
    assert!(
        interval(Build::GRCh38, 1, u32::MAX, u32::MAX)
            .inclusive_bounds()
            .is_none()
    );
}

#[test]
fn allele_shape_requires_dna_and_reference_length() {
    let interval = |end| {
        GenomicInterval::new(
            Build::GRCh38,
            Chromosome::new(1).unwrap(),
            Span0::new(0, end).unwrap(),
        )
    };
    for (end, reference, alternate, valid) in [
        (0, vec![], vec![], false),
        (0, vec![], vec![Base::A], true),
        (1, vec![Base::A], vec![], true),
        (1, vec![Base::A], vec![Base::A], true),
        (2, vec![Base::A], vec![Base::T], false),
    ] {
        let key = AlleleKey::new(interval(end), reference.clone(), alternate.clone());
        assert_eq!(key.is_ok(), valid);
        if let Ok(key) = key {
            assert_eq!(key.interval().unwrap(), interval(end));
            assert_eq!(key.reference(), reference);
            assert_eq!(key.alternate(), alternate);
        }
    }
}

#[test]
fn construction_failures_have_stable_codes() {
    for (error, code) in [
        (Position1::new(0).unwrap_err(), "coord.position.zero"),
        (
            Position1::new(u32::MAX)
                .unwrap()
                .checked_add(1)
                .unwrap_err(),
            "coord.position.overflow",
        ),
        (Span0::new(2, 1).unwrap_err(), "coord.span.order"),
        (
            Span0::from_offset_count(u32::MAX, 1).unwrap_err(),
            "coord.span.overflow",
        ),
        (
            Span0::new(0, 2).unwrap().check_bounds(1).unwrap_err(),
            "coord.span.outside",
        ),
        (
            "bad".parse::<Position1>().unwrap_err(),
            "coord.position.invalid",
        ),
        (
            CoordinateError::BuildMismatch,
            "coord.interval.build_mismatch",
        ),
        (
            CoordinateError::ChromosomeMismatch,
            "coord.interval.chromosome_mismatch",
        ),
        (
            CoordinateError::ReferenceLengthMismatch,
            "allele.reference.length",
        ),
        (CoordinateError::InvalidAlleleShape, "allele.state.empty"),
    ] {
        assert_eq!(error.code(), code);
        assert_eq!(error.to_string(), code);
    }
}
