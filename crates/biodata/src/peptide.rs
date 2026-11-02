//! Describe an already translated peptide difference without sequence lookup.
use crate::{
    Residue,
    hgvs::{Distance, Extension, ProteinChange, ProteinRange, ProteinSite},
};

/// How the nucleotide edit affects the translated frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frame {
    /// Reading frame is preserved.
    Preserved,
    /// Reading frame shifts after the edit.
    Shifted,
    /// A stop encoded by the inserted bases bounds the replacement.
    Terminated(usize),
}

/// The translated peptide lacks a required site or span.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeptideError {
    /// An input bound contradicts the supplied peptides.
    Input,
    /// The described change needs unavailable peptide context.
    Unsupported,
}
impl PeptideError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Input => "peptide.input",
            Self::Unsupported => "peptide.context.unsupported",
        }
    }
}
impl std::fmt::Display for PeptideError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for PeptideError {}

/// Describe a peptide change from translated reference and alternate residues.
///
/// `affected` gives the codons touched by the nucleotide edit and
/// `expected_length` gives the expected alternate peptide length in codons.
#[expect(
    clippy::too_many_lines,
    reason = "preserve the reviewed parser layout and hot-path behavior"
)]
pub fn difference(
    original: &[Residue],
    alternate: &[Residue],
    affected: std::ops::Range<usize>,
    expected_length: i64,
    frame: Frame,
) -> Result<ProteinChange, PeptideError> {
    let first = original
        .iter()
        .zip(alternate)
        .take_while(|(a, b)| a == b)
        .count();
    if first >= original.len() {
        let index = affected
            .end
            .saturating_sub(1)
            .min(original.len().checked_sub(1).ok_or(PeptideError::Input)?);
        return Ok(ProteinChange::Same(Some(site(original, index)?)));
    }
    if first == 0 {
        return Ok(ProteinChange::Unknown(None));
    }
    let from = site(original, first)?;
    let Some(&to) = alternate.get(first) else {
        return Ok(ProteinChange::Unknown(None));
    };
    if from.residue == Residue::Ter {
        let stop = alternate
            .iter()
            .skip(first + 1)
            .position(|aa| *aa == Residue::Ter)
            .map_or(Distance::Unknown, |n| Distance::Known(n as u32 + 1));
        return Ok(ProteinChange::Extension(Extension::Stop {
            site: from,
            to,
            stop,
        }));
    }
    if to == Residue::Ter {
        return Ok(ProteinChange::Substitution { site: from, to });
    }
    if let Frame::Terminated(stop) = frame {
        return replacement(
            original,
            first,
            affected.end.max(first),
            alternate
                .get(first..=stop)
                .ok_or(PeptideError::Input)?
                .to_vec(),
        );
    }
    if frame == Frame::Shifted {
        let stop = alternate
            .iter()
            .skip(first)
            .position(|aa| *aa == Residue::Ter)
            .map_or(Distance::Unknown, |n| Distance::Known(n as u32 + 1));
        return Ok(ProteinChange::Frameshift {
            site: from,
            to: Some(to),
            stop: Some(stop),
        });
    }
    let expected_end = usize::try_from(expected_length).map_err(|_| PeptideError::Input)?;
    let normalization_end = if original.last() == Some(&Residue::Ter)
        && alternate.get(expected_end.saturating_sub(1)) != Some(&Residue::Ter)
    {
        alternate
            .iter()
            .position(|aa| *aa == Residue::Ter)
            .ok_or(PeptideError::Unsupported)?
            + 1
    } else {
        expected_end.min(alternate.len())
    };
    let alternate = alternate
        .get(..normalization_end)
        .ok_or(PeptideError::Input)?;
    let shared = original
        .iter()
        .skip(first)
        .rev()
        .zip(alternate.iter().skip(first).rev())
        .take_while(|(a, b)| a == b)
        .count();
    let end = original.len() - shared;
    let mut inserted = alternate
        .get(first..alternate.len() - shared)
        .ok_or(PeptideError::Input)?
        .to_vec();
    if let Some(stop) = inserted.iter().position(|aa| *aa == Residue::Ter) {
        inserted.truncate(stop + 1);
    }
    replacement(original, first, end, inserted)
}

/// Describe a bounded peptide replacement after callers have selected its policy window.
pub fn replacement(
    original: &[Residue],
    first: usize,
    end: usize,
    inserted: Vec<Residue>,
) -> Result<ProteinChange, PeptideError> {
    if first > end || end > original.len() || (first == end && (first == 0 || inserted.is_empty()))
    {
        return Err(PeptideError::Input);
    }
    let range = |start: usize, end: usize| -> Result<ProteinRange, PeptideError> {
        Ok(ProteinRange {
            start: site(original, start)?,
            end: (end > start + 1)
                .then(|| site(original, end - 1))
                .transpose()?,
        })
    };
    if first == end {
        if first >= inserted.len()
            && original.get(first - inserted.len()..first) == Some(inserted.as_slice())
        {
            return Ok(ProteinChange::Duplication(range(
                first - inserted.len(),
                first,
            )?));
        }
        return Ok(ProteinChange::Insertion {
            after: site(original, first - 1)?,
            before: site(original, first)?,
            inserted,
        });
    }
    if inserted.is_empty() {
        return Ok(ProteinChange::Deletion(range(first, end)?));
    }
    if end == first + 1 && inserted.len() == 1 {
        return Ok(ProteinChange::Substitution {
            site: site(original, first)?,
            to: *inserted.first().ok_or(PeptideError::Input)?,
        });
    }
    Ok(ProteinChange::Delins {
        range: range(first, end)?,
        inserted,
    })
}

/// Name a residue in a translated peptide by its zero-based slice index.
pub fn site(original: &[Residue], index: usize) -> Result<ProteinSite, PeptideError> {
    Ok(ProteinSite {
        residue: *original.get(index).ok_or(PeptideError::Unsupported)?,
        position: crate::Position1::new(
            u32::try_from(index.checked_add(1).ok_or(PeptideError::Input)?)
                .map_err(|_| PeptideError::Input)?,
        )
        .map_err(|_| PeptideError::Input)?,
    })
}
