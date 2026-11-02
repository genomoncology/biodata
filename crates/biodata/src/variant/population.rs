//! Source population strata, without computed frequencies or classification.
use crate::{Attribution, Build, Fraction, Text, VariantRef};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// One checked stratum of one source dataset release.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Fields"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct PopulationFrequency(Fields);
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    subject: VariantRef,
    build: Build,
    dataset: Text,
    dataset_version: Text,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    data_type: Option<PopulationDataType>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    ancestry_group: Option<Text>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    sex: Option<PopulationSex>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    subset: Option<Text>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    allele_count: Option<u64>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    allele_number: Option<u64>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    allele_frequency: Option<Fraction>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    homozygote_count: Option<u64>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    hemizygote_count: Option<u64>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    faf95: Option<Fraction>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
    )]
    filters: Box<[Text]>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    genotype_filter: Option<GenotypeFilter>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    grpmax_group: Option<Text>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    heterozygote_count: Option<u64>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    homoplasmic_count: Option<u64>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    heteroplasmic_count: Option<u64>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    homoplasmic_frequency: Option<Fraction>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    heteroplasmic_frequency: Option<Fraction>,
    attribution: Attribution,
}
impl TryFrom<Fields> for PopulationFrequency {
    type Error = crate::Error;
    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        if fields
            .allele_count
            .zip(fields.allele_number)
            .is_some_and(|(ac, an)| ac > an)
        {
            return Err(crate::Error::PopulationCountExceedsNumber);
        }
        if fields.ancestry_group.is_some() && fields.grpmax_group.is_some() {
            return Err(crate::Error::PopulationGrpmaxMisplaced);
        }
        let has_mt_field = fields.homoplasmic_count.is_some()
            || fields.heteroplasmic_count.is_some()
            || fields.homoplasmic_frequency.is_some()
            || fields.heteroplasmic_frequency.is_some();
        if has_mt_field
            && fields.subject.allele_accession().is_some_and(|accession| {
                crate::Chromosome::from_accession(accession, fields.build)
                    .is_ok_and(|chromosome| chromosome.number() != 25)
            })
        {
            return Err(crate::Error::PopulationMitochondrialFieldMisplaced);
        }
        Ok(Self(fields))
    }
}
impl PopulationFrequency {
    /// Source-reported subject.
    pub fn subject(&self) -> &VariantRef {
        &self.0.subject
    }
    /// Source-reported build.
    pub fn build(&self) -> Build {
        self.0.build
    }
    /// Source-reported dataset.
    pub fn dataset(&self) -> &Text {
        &self.0.dataset
    }
    /// Source-reported dataset version.
    pub fn dataset_version(&self) -> &Text {
        &self.0.dataset_version
    }
    /// Source-reported data type.
    pub fn data_type(&self) -> Option<PopulationDataType> {
        self.0.data_type
    }
    /// Source-reported ancestry group.
    pub fn ancestry_group(&self) -> Option<&Text> {
        self.0.ancestry_group.as_ref()
    }
    /// Source-reported sex.
    pub fn sex(&self) -> Option<PopulationSex> {
        self.0.sex
    }
    /// Source-reported subset.
    pub fn subset(&self) -> Option<&Text> {
        self.0.subset.as_ref()
    }
    /// Source-reported allele count.
    pub fn allele_count(&self) -> Option<u64> {
        self.0.allele_count
    }
    /// Source-reported allele number.
    pub fn allele_number(&self) -> Option<u64> {
        self.0.allele_number
    }
    /// Source-reported allele frequency.
    pub fn allele_frequency(&self) -> Option<Fraction> {
        self.0.allele_frequency
    }
    /// Source-reported homozygote count.
    pub fn homozygote_count(&self) -> Option<u64> {
        self.0.homozygote_count
    }
    /// Source-reported hemizygote count.
    pub fn hemizygote_count(&self) -> Option<u64> {
        self.0.hemizygote_count
    }
    /// Source-reported faf95.
    pub fn faf95(&self) -> Option<Fraction> {
        self.0.faf95
    }
    /// Source-reported filters.
    pub fn filters(&self) -> &[Text] {
        &self.0.filters
    }
    /// Source-reported genotype filter.
    pub fn genotype_filter(&self) -> Option<GenotypeFilter> {
        self.0.genotype_filter
    }
    /// Source-reported grpmax group.
    pub fn grpmax_group(&self) -> Option<&Text> {
        self.0.grpmax_group.as_ref()
    }
    /// Source-reported heterozygote count.
    pub fn heterozygote_count(&self) -> Option<u64> {
        self.0.heterozygote_count
    }
    /// Source-reported homoplasmic count.
    pub fn homoplasmic_count(&self) -> Option<u64> {
        self.0.homoplasmic_count
    }
    /// Source-reported heteroplasmic count.
    pub fn heteroplasmic_count(&self) -> Option<u64> {
        self.0.heteroplasmic_count
    }
    /// Source-reported homoplasmic frequency.
    pub fn homoplasmic_frequency(&self) -> Option<Fraction> {
        self.0.homoplasmic_frequency
    }
    /// Source-reported heteroplasmic frequency.
    pub fn heteroplasmic_frequency(&self) -> Option<Fraction> {
        self.0.heteroplasmic_frequency
    }
    /// Source-reported attribution.
    pub fn attribution(&self) -> &Attribution {
        &self.0.attribution
    }
    /// Preserve one dataset identity without inventing observations.
    pub fn new(
        subject: VariantRef,
        build: Build,
        dataset: Text,
        dataset_version: Text,
        attribution: Attribution,
    ) -> Self {
        Self(Fields {
            subject,
            build,
            dataset,
            dataset_version,
            attribution,
            data_type: None,
            ancestry_group: None,
            sex: None,
            subset: None,
            allele_count: None,
            allele_number: None,
            allele_frequency: None,
            homozygote_count: None,
            hemizygote_count: None,
            faf95: None,
            genotype_filter: None,
            grpmax_group: None,
            heterozygote_count: None,
            homoplasmic_count: None,
            heteroplasmic_count: None,
            homoplasmic_frequency: None,
            heteroplasmic_frequency: None,
            filters: Box::default(),
        })
    }
    /// Preserve source stratum labels without combining or remapping groups.
    pub fn with_stratum(
        mut self,
        data_type: Option<PopulationDataType>,
        ancestry_group: Option<Text>,
        sex: Option<PopulationSex>,
        subset: Option<Text>,
    ) -> Result<Self, crate::Error> {
        self.0.data_type = data_type;
        self.0.ancestry_group = ancestry_group;
        self.0.sex = sex;
        self.0.subset = subset;
        self.0.try_into()
    }
    /// Preserve source counts while refusing an allele count above its denominator.
    pub fn with_counts(
        mut self,
        allele_count: Option<u64>,
        allele_number: Option<u64>,
        homozygote_count: Option<u64>,
        hemizygote_count: Option<u64>,
    ) -> Result<Self, crate::Error> {
        self.0.allele_count = allele_count;
        self.0.allele_number = allele_number;
        self.0.homozygote_count = homozygote_count;
        self.0.hemizygote_count = hemizygote_count;
        self.0.try_into()
    }
    /// Preserve every source filter in native order.
    pub fn with_filters(mut self, filters: Box<[Text]>) -> Self {
        self.0.filters = filters;
        self
    }
}
crate::wire_enum! {
    /// Dataset sequencing sample set.
    PopulationDataType {
        /// Exome samples.
        Exomes => "exomes",
        /// Genome samples.
        Genomes => "genomes",
        /// Joint samples.
        Joint => "joint",
    }
}
crate::wire_enum! {
    /// Sex stratum stated by a population source.
    PopulationSex {
        /// Source XX stratum.
        Xx => "xx",
        /// Source XY stratum.
        Xy => "xy",
    }
}

crate::wire_enum! {
    /// Source count filtering state.
    GenotypeFilter {
        /// Source counts before its genotype filters.
        Raw => "raw",
    }
}

#[cfg(feature = "serde")]
crate::serde_object::object_deserialize!(PopulationFrequency, Fields);
