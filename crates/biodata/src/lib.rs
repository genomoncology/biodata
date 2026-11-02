#![doc = include_str!("../README.md")]

mod frozen_api;
mod serde_object;

pub use id::{Doi, EuropePmcId, Pmcid, Pmid, SemanticScholarId};

mod source_identifier;
pub use source_identifier::{Identifier, IdentifierNamespace};

mod source_text_value;
pub use source_text_value::SourceText;
mod ontology_id;
mod source_text;

pub use ontology_id::{MondoId, NcitId};

pub use source_text::{HgvsText, PartialDate, SourceTextError, Text, TextError};

mod error;
mod id;
mod macros;
#[cfg(feature = "schema")]
#[doc(hidden)]
pub mod schema_pattern;

pub use error::Error;
pub use id::{Cytoband, EnsemblGeneId, UniProtAcc};

mod gene;
pub use gene::{
    Gene, GeneLocation, GeneLocationKind, GeneStatus, HgncStatus, LocusGroup, LocusType, MatchKind,
    SourceGeneRef, Summary, SummarySource,
};

mod markdown;
pub use markdown::Markdown;

mod variant;
pub use variant::{Representation, Variant};

pub use variant::CodingChange;

pub use variant::{Expression, ExpressionError, ExpressionSyntax};

mod trial;

pub use trial::{NctId, StudyType, TrialPhase, TrialSex, TrialStatus};

mod trial_detail;
pub use trial_detail::{
    TrialAgeBound, TrialArm, TrialCard, TrialEligibility, TrialIntervention, TrialOutcome,
    TrialOutcomeKind,
};

pub use trial_detail::{TrialDate, TrialDateKind};

pub use trial_detail::{TrialArmKind, TrialInterventionKind};

mod trial_sites;
pub use trial_sites::{
    TrialContact, TrialContactRole, TrialCoordinates, TrialLocation, TrialOfficial,
    TrialOfficialRole, TrialSiteStatus,
};

mod trial_publications;

pub use trial_publications::{TrialDocument, TrialReference, TrialReferenceKind};
mod trial_upload_time;
pub use trial_upload_time::TrialUploadTime;

pub use variant::{
    AcmgAmp2015Code, AmpAscoCap2017LevelCode, AmpAscoCap2017TierCode, Attribution,
    CivicEvidenceLevelCode, ClingenCgcVicc2022Code, ClingenLowPenetrance2024Code, ClinvarHaplotype,
    ClinvarHaplotypeMember, Coding, Concept, Curie, Direction, OncokbLevelCode,
    OncokbOncogenicityCode, Predicate, ProfileExpression, ProfileNode, Proposition, Statement,
    Term, VariantRef,
};

pub use variant::{GenotypeFilter, PopulationDataType, PopulationFrequency, PopulationSex};

mod number;
pub use number::{Fraction, WithUnit};

pub use variant::PredictorScore;

mod license;
pub use license::License;

mod disease;
pub use disease::{
    DiseaseAssociation, DiseaseAssociationQualifier, DiseaseAssociationReference, DiseaseEndpoint,
    DiseaseRelation, DiseaseStatistic, DiseaseSynonym, StatisticEstimate, StatisticSelector,
};

mod drug;
pub use drug::{
    AuthorizationDate, AuthorizationEvent, DrugDiseaseReference, DrugId, DrugIdentity,
    DrugIdentityLevel, DrugIdentityRelation, DrugIndicationClaim, DrugInteractionClaim,
    DrugLabelReference, DrugLabelSection, DrugNativeReference, DrugNativeTarget, DrugProduct,
    DrugStage, DrugTargetRelation,
};

mod adverse_event;
pub use adverse_event::{
    FaersAgeUnit, FaersDrug, FaersIdentifiers, FaersPatient, FaersReaction, FaersReport,
    FaersReporter, FaersSeriousness, MaudeDevice, MaudeNarrative, MaudeReport, ReportId,
};

mod pgx;
pub use pgx::*;
mod diagnostic;
pub use diagnostic::*;

mod gwas;
pub use gwas::{GwasAssociationId, GwasStudyId, GwasTraitId, ScientificPValue};
mod phenotype;
pub use phenotype::*;
mod pathway;
pub use pathway::{
    Pathway, PathwayEventRef, PathwayId, PathwayLiteratureRef, PathwayMember, PathwayParticipant,
    PathwayRef, PathwaySourceRef, PathwaySpecies, PathwaySummation, ReactomeId, WikiPathwaysId,
};

mod protein;
pub use protein::*;
mod study;
pub use study::*;

mod cell_line;
pub use cell_line::*;

mod author;
pub use author::*;

/// Omit an optional collection when the source supplied no values.
pub fn absent_or_empty<T>(value: &Option<Box<[T]>>) -> bool {
    value.as_ref().is_none_or(|items| items.is_empty())
}

pub mod alteration;

pub mod hgvs;

pub mod iscn;

mod outcome;

pub use outcome::{Code, InputCode, Outcome, Parsed, Refusal, Span};

mod parse;

pub use parse::{MAX_LEN, parse_bytes};

mod residue;

pub use residue::Residue;

mod table;

mod value;

pub use value::{
    Accession, AccessionBase, AccessionCode, Base, EnsemblFeature, LrgPart, RefSeqPrefix,
};

mod genetic_code;

pub use genetic_code::GeneticCode;

mod inheritance;

pub use inheritance::ModeOfInheritance;

mod transcript_region;

pub use transcript_region::{Region, RegionKind, TranscriptRegion, TranscriptRegionKind};

mod transcript_position;

pub use transcript_position::TranscriptPosition;

mod reference;

pub use reference::{
    Build, Chromosome, Contig, ReferenceError, Strand, parse_chromosome, without_chr_prefix,
};

mod coordinate;

pub use coordinate::{AlleleKey, CoordinateError, GenomicInterval, Position1, Span0};

mod uncertain;

pub use uncertain::{Bound, UncertainLocation, UncertainSpan0};

mod copy_number;

pub use copy_number::{CopyChange, CopyNumberChange, CopyNumberCount};

mod protein_site;

pub use protein_site::{ProteinSite, ProteinSiteError, TranscriptReference};

mod sequence;

pub use sequence::{Allele, Sequence, SequenceLocation, reverse_complement, trim_flanks};

mod transcript;

pub use transcript::{
    AlignedExon, AlignmentError, Cigar, CigarCode, CigarOp, GeneRef, Mane, ManeStatus, Transcript,
    TranscriptAlignment, TranscriptError,
};

pub mod vcf;

mod edit;

pub use edit::EditKind;

mod lift;

pub use lift::{LiftFailure, LiftOutcome, Lifted};

mod annotation;

pub use annotation::{
    Consequence, ConsequenceError, GeneExtent, NmdEscape, NmdEscapeReason, NmdRuleId, SoTerm,
    TranscriptAccession, TranscriptConsequence,
};

mod uniprot_feature;

pub use uniprot_feature::{
    AminoAcids, AminoAcidsError, UniProtFeature, UniProtIsoform, UniProtIsoformError,
};

pub mod peptide;

mod identifier;
pub use identifier::{
    CaId, ClinVarAlleleId, ClinVarVariationId, CosmicId, CosmicKind, HgncId, IdError, NcbiGeneId,
    OmimId, PaId, RcvId, RsId, ScvId, VariantId, VcvId, VrsId,
};

mod source_shapes;
pub use source_shapes::{
    CohortObservation, CohortUnit, ConditionCount, FiniteNumber, Method, NumberError, Subject,
    SubjectError, TranscriptId,
};

mod population_frequency;
pub use population_frequency::FrequencyError;
mod clinvar_value;
pub use clinvar_value::{AlleleOrigin, ClassificationType, ClinVarValueError, ReviewStatus};

mod gene_symbol;
pub use gene_symbol::GeneSymbol;

mod coding;
pub use coding::{
    ClingenDosageScoreCode, ClingenGeneValidity2017Code, ClinvarClassificationCode, CodingError,
    HpoInheritanceCode, SoCode,
};

mod text_control;

#[cfg(feature = "schema")]
mod schema_compat;

pub use variant::{Category, VariantRefError};

mod alteration_reference;
pub use alteration_reference::AlterationReferenceError;

pub use source_shapes::SourceShapeError;

#[doc(hidden)]
#[cfg(feature = "schema")]
pub use schemars as __schemars;
#[doc(hidden)]
#[cfg(feature = "serde")]
pub use serde as __serde;

mod text_literal;
pub use text_literal::TextLiteral;
