use crate::{ClinPgxAccession, Curie, EnsemblGeneId, GeneSymbol, HgncId, NcbiGeneId, Pmid};
#[cfg(feature = "serde")]
use serde::Serialize;
use std::collections::BTreeMap;
crate::serde_object::object_record! {
    /// Source-native PgxGeneIdentity fields; no clinical interpretation.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct PgxGeneIdentity {
        /// Native `symbol`; literal source value.
        pub symbol: GeneSymbol,
        /// Native `hgncid`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub hgncid: Option<HgncId>,
        /// Native `ncbiid`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub ncbiid: Option<NcbiGeneId>,
        /// Native `ensemblid`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub ensemblid: Option<EnsemblGeneId>,
        /// Native `clinpgxid`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub clinpgxid: Option<ClinPgxAccession>,
        /// Native `lookupmethod`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub lookupmethod: Option<Box<str>>,
        /// Native `version`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub version: Option<i64>,
    }
}
crate::serde_object::object_record! {
    /// Source-native PgxPair fields; no clinical interpretation.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct PgxPair {
        /// Native `pairid`; literal source value.
        pub pairid: crate::Identifier<i64>,
        /// Native `drugid`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub drugid: Option<Curie>,
        /// Native `drugname`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub drugname: Option<Box<str>>,
        /// Native `genesymbol`; literal source value.
        pub genesymbol: GeneSymbol,
        /// Native `guidelinename`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub guidelinename: Option<Box<str>>,
        /// Native `guidelineurl`; literal source value.
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub guidelineurl: Option<Box<str>>,
        /// Native `cpiclevel`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub cpiclevel: Option<Box<str>>,
        /// Native `clinpgxlevel`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub clinpgxlevel: Option<Box<str>>,
        /// Native `pgxtesting`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub pgxtesting: Option<Box<str>>,
        /// Native `pmids`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub pmids: Option<Box<[Pmid]>>,
        /// Native `usedforrecommendation`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub usedforrecommendation: Option<Box<str>>,
        /// Native `provisional`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub provisional: Option<bool>,
    }
}
crate::serde_object::object_record! {
    /// Source-native PgxRecommendation fields; no clinical interpretation.
    /// ```compile_fail
    /// fn interpret(value: biodata::PgxRecommendation) -> biodata::Statement { value }
    /// ```
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct PgxRecommendation {
        /// Native `recommendationid`; literal source value.
        pub recommendationid: crate::Identifier<i64>,
        /// Native `lookupkey`; literal source value.
        pub lookupkey: BTreeMap<super::PgxGeneKey, Box<str>>,
        /// Native `drugname`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub drugname: Option<Box<str>>,
        /// Native `guidelinename`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub guidelinename: Option<Box<str>>,
        /// Native `guidelineurl`; literal source value.
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub guidelineurl: Option<Box<str>>,
        /// Native `implications`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub implications: Option<BTreeMap<super::PgxGeneKey, Box<str>>>,
        /// Native `drugrecommendation`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub drugrecommendation: Option<Box<str>>,
        /// Native `classification`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub classification: Option<Box<str>>,
        /// Native `phenotypes`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub phenotypes: Option<BTreeMap<super::PgxGeneKey, Box<str>>>,
        /// Native `activityscore`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub activityscore: Option<BTreeMap<super::PgxGeneKey, Box<str>>>,
        /// Native `population`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub population: Option<Box<str>>,
        /// Native `comments`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub comments: Option<Box<str>>,
    }
}
crate::serde_object::object_record! {
    /// Source-native PgxDrugRecommendation fields; no clinical interpretation.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct PgxDrugRecommendation {
        /// Native `id`; literal source value.
        pub id: crate::Identifier<i64>,
        /// Native `guidelineid`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub guidelineid: Option<crate::Identifier<i64>>,
        /// Native `drugid`; literal source value.
        pub drugid: Curie,
        /// Native `implications`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub implications: Option<BTreeMap<super::PgxGeneKey, Box<str>>>,
        /// Native `drugrecommendation`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub drugrecommendation: Option<Box<str>>,
        /// Native `classification`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub classification: Option<Box<str>>,
        /// Native `phenotypes`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub phenotypes: Option<BTreeMap<super::PgxGeneKey, Box<str>>>,
        /// Native `activityscore`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub activityscore: Option<BTreeMap<super::PgxGeneKey, Box<str>>>,
        /// Native `allelestatus`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub allelestatus: Option<BTreeMap<super::PgxGeneKey, Box<str>>>,
        /// Native `lookupkey`; literal source value.
        pub lookupkey: BTreeMap<super::PgxGeneKey, Box<str>>,
        /// Native `population`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub population: Option<Box<str>>,
        /// Native `comments`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub comments: Option<Box<str>>,
        /// Native `version`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub version: Option<i64>,
        /// Native `dosinginformation`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub dosinginformation: Option<bool>,
        /// Native `alternatedrugavailable`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub alternatedrugavailable: Option<bool>,
        /// Native `otherprescribingguidance`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub otherprescribingguidance: Option<bool>,
    }
}
crate::serde_object::object_record! {
    /// Source-native PgxGuidelineGene fields; no clinical interpretation.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct PgxGuidelineGene {
        /// Native `url`; literal source value.
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub url: Option<Box<str>>,
        /// Native `symbol`; literal source value.
        pub symbol: GeneSymbol,
    }
}
crate::serde_object::object_record! {
    /// Source-native PgxGuidelineReference fields; no clinical interpretation.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct PgxGuidelineReference {
        /// Native `guideline_name`; literal source value.
        pub guideline_name: Box<str>,
        /// Native `guideline_url`; literal source value.
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub guideline_url: Option<Box<str>>,
        /// Native `drugs`; literal source value.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub drugs: Box<[Box<str>]>,
        /// Native `genes`; literal source value.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub genes: Box<[PgxGuidelineGene]>,
    }
}
crate::serde_object::object_record! {
    /// Source-native PgxDiplotypeResult fields; no clinical interpretation.
    /// ```compile_fail
    /// fn interpret(value: biodata::PgxDiplotypeResult) -> biodata::Statement { value }
    /// ```
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct PgxDiplotypeResult {
        /// Native `genesymbol`; literal source value.
        pub genesymbol: GeneSymbol,
        /// Native `diplotype`; literal source value.
        pub diplotype: Box<str>,
        /// Native `function1`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub function1: Option<Box<str>>,
        /// Native `function2`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub function2: Option<Box<str>>,
        /// Native `activityvalue1`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub activityvalue1: Option<Box<str>>,
        /// Native `activityvalue2`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub activityvalue2: Option<Box<str>>,
        /// Native `totalactivityscore`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub totalactivityscore: Option<Box<str>>,
        /// Native `description`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub description: Option<Box<str>>,
        /// Native `diplotypekey`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub diplotypekey: Option<BTreeMap<super::PgxGeneKey, BTreeMap<Box<str>, u32>>>,
        /// Native `generesult`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub generesult: Option<Box<str>>,
        /// Native `ehrpriority`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub ehrpriority: Option<Box<str>>,
        /// Native `consultationtext`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub consultationtext: Option<Box<str>>,
        /// Native `lookupkey`; literal source value.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub lookupkey: Option<BTreeMap<super::PgxGeneKey, Box<str>>>,
    }
}
