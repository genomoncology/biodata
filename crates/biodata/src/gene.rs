//! Checked human gene identity and source-stated annotations.

mod annotations;
mod values;
pub use annotations::Summary;
pub use values::{GeneStatus, HgncStatus, LocusGroup, LocusType, MatchKind, SummarySource};

mod model;
pub use model::Gene;

mod reference;
pub use reference::SourceGeneRef;

mod location;
pub use location::{GeneLocation, GeneLocationKind};
