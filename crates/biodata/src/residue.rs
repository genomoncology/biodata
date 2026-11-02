//! Amino acid residues shared by protein notations.
use crate::parse::{Input, Result, step};
use std::fmt;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::token::{any, take};

// Each canonical spelling and its one-letter alias are declared together.
macro_rules! residues {
    ($($variant:ident => $letter:literal),+ $(,)?) => {
        /// An amino acid, ambiguous residue, or termination marker.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Residue {
            $(#[doc = concat!("`", stringify!($variant), "`.")]
            $variant,)+
        }
        impl fmt::Display for Residue {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if f.alternate() {
                    match self { $(Self::$variant => f.write_str(concat!($letter)),)+ }
                } else {
                    f.write_str(match self { $(Self::$variant => stringify!($variant),)+ })
                }
            }
        }
        pub(crate) fn residue(input: &mut Input<'_>) -> Result<Residue> {
            step(input)?;
            alt((
                take(3usize).verify_map(|text: &str| match text {
                    $(stringify!($variant) => Some(Residue::$variant),)+
                    _ => None,
                }),
                any.verify_map(|letter| match letter {
                    $($letter => Some(Residue::$variant),)+
                    _ => None,
                }),
            )).parse_next(input)
        }
    };
}
residues! {
    Ala => 'A', Arg => 'R', Asn => 'N', Asp => 'D', Cys => 'C',
    Gln => 'Q', Glu => 'E', Gly => 'G', His => 'H', Ile => 'I',
    Leu => 'L', Lys => 'K', Met => 'M', Phe => 'F', Pro => 'P',
    Ser => 'S', Thr => 'T', Trp => 'W', Tyr => 'Y', Val => 'V',
    Sec => 'U', Pyl => 'O', Ter => '*', Asx => 'B', Glx => 'Z', Xaa => 'X',
}
