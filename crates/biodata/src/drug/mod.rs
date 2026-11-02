//! Source-native drug records without consumer requests, policy or I/O.
mod claims;
mod id;
mod identity;
mod interaction;
mod native;
mod regulatory;
mod target;
pub use claims::{DrugIndicationClaim, DrugLabelReference, DrugLabelSection};
pub use id::DrugId;
pub use identity::{DrugIdentity, DrugIdentityLevel, DrugIdentityRelation, DrugProduct};
pub use regulatory::{AuthorizationDate, AuthorizationEvent};

pub use interaction::DrugInteractionClaim;
pub use native::{DrugDiseaseReference, DrugNativeReference, DrugNativeTarget, DrugStage};
pub use target::DrugTargetRelation;
