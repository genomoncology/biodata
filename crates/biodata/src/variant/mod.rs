//! Source-qualified variant values.
mod model;
pub use model::{Representation, Variant};
mod notation;
pub use notation::{Expression, ExpressionError, ExpressionSyntax};
mod coding_change;
pub use coding_change::CodingChange;
mod predictor;
pub use predictor::PredictorScore;
mod statement;
pub use statement::*;
mod population;
pub use population::{GenotypeFilter, PopulationDataType, PopulationFrequency, PopulationSex};
