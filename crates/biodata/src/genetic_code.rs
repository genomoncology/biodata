//! [NCBI translation tables 1 and 2](https://www.ncbi.nlm.nih.gov/Taxonomy/Utils/wprintgc.cgi)
//! for DNA codons. A caller chooses the table from its own source context.
use crate::Residue;

/// A supported NCBI translation table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeneticCode {
    /// Standard nuclear code, NCBI table 1.
    Standard,
    /// Vertebrate mitochondrial code, NCBI table 2.
    VertebrateMitochondrial,
}
impl GeneticCode {
    /// Select a supported NCBI translation table by number.
    pub const fn from_ncbi_table(number: u8) -> Option<Self> {
        match number {
            1 => Some(Self::Standard),
            2 => Some(Self::VertebrateMitochondrial),
            _ => None,
        }
    }
    /// NCBI translation table number.
    pub const fn ncbi_table(self) -> u8 {
        match self {
            Self::Standard => 1,
            Self::VertebrateMitochondrial => 2,
        }
    }
    /// Translate an exact upper-case DNA codon. Unrecognized text yields Xaa.
    pub fn exact_residue(self, codon: &[u8]) -> Residue {
        if self == Self::VertebrateMitochondrial {
            match codon {
                b"ATA" => return Residue::Met,
                b"TGA" => return Residue::Trp,
                b"AGA" | b"AGG" => return Residue::Ter,
                _ => {}
            }
        }
        use Residue::*;
        match codon {
            b"TTT" | b"TTC" => Phe,
            b"TTA" | b"TTG" | b"CTT" | b"CTC" | b"CTA" | b"CTG" => Leu,
            b"ATT" | b"ATC" | b"ATA" => Ile,
            b"ATG" => Met,
            b"GTT" | b"GTC" | b"GTA" | b"GTG" => Val,
            b"TCT" | b"TCC" | b"TCA" | b"TCG" | b"AGT" | b"AGC" => Ser,
            b"CCT" | b"CCC" | b"CCA" | b"CCG" => Pro,
            b"ACT" | b"ACC" | b"ACA" | b"ACG" => Thr,
            b"GCT" | b"GCC" | b"GCA" | b"GCG" => Ala,
            b"TAT" | b"TAC" => Tyr,
            b"TAA" | b"TAG" | b"TGA" => Ter,
            b"CAT" | b"CAC" => His,
            b"CAA" | b"CAG" => Gln,
            b"AAT" | b"AAC" => Asn,
            b"AAA" | b"AAG" => Lys,
            b"GAT" | b"GAC" => Asp,
            b"GAA" | b"GAG" => Glu,
            b"TGT" | b"TGC" => Cys,
            b"TGG" => Trp,
            b"CGT" | b"CGC" | b"CGA" | b"CGG" | b"AGA" | b"AGG" => Arg,
            b"GGT" | b"GGC" | b"GGA" | b"GGG" => Gly,
            _ => Xaa,
        }
    }
    /// Translate an exact or ambiguous upper-case IUPAC DNA codon.
    /// Return Xaa when possible exact codons disagree.
    pub fn residue(self, codon: &[u8; 3]) -> Residue {
        let exact = self.exact_residue(codon);
        if exact != Residue::Xaa {
            return exact;
        }
        let [a, b, c] = codon;
        let mut result = None;
        for a in possibilities(a) {
            for b in possibilities(b) {
                for c in possibilities(c) {
                    let next = self.exact_residue(&[*a, *b, *c]);
                    if result.is_some_and(|old| old != next) {
                        return Residue::Xaa;
                    }
                    result = Some(next);
                }
            }
        }
        result.unwrap_or(Residue::Xaa)
    }
}

fn possibilities(base: &u8) -> &[u8] {
    match base {
        b'R' => b"AG",
        b'Y' => b"CT",
        b'S' => b"CG",
        b'W' => b"AT",
        b'K' => b"GT",
        b'M' => b"AC",
        b'B' => b"CGT",
        b'D' => b"AGT",
        b'H' => b"ACT",
        b'V' => b"ACG",
        b'N' => b"ACGT",
        _ => std::slice::from_ref(base),
    }
}
