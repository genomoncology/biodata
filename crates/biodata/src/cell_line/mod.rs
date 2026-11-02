//! Source-owned cell line records, without I/O or allele interpretation.
mod id;
pub use id::*;
mod records;
pub use records::*;

mod sex;
pub use sex::CellLineSex;
