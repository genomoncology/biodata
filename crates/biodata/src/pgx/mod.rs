//! Native pharmacogenomics records without I/O, clinical propositions or patient inference.
mod accession;
mod annotation;
mod cpic;
mod detail;
pub use accession::{ClinPgxAccession, PgxDiplotypeExpression, PgxGeneKey};
pub use annotation::*;
pub use cpic::*;
pub use detail::*;
