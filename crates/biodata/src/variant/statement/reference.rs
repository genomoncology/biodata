use crate::{ClinvarHaplotype, HgvsText, ProfileExpression, SourceGeneRef, Text, VariantId};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
/// Checked prefix:local ontology identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Curie(Box<str>);
impl Curie {
    /// Borrow the unchanged source identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub(crate) fn qualified_parts(value: &str) -> Option<(&str, &str)> {
        let (prefix, local) = value.split_once(':')?;
        (!prefix.is_empty()
            && !local.is_empty()
            && prefix
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.'))
            && !value.chars().any(|c| c.is_whitespace() || c.is_control()))
        .then_some((prefix, local))
    }
}
impl FromStr for Curie {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (_, local) = Self::qualified_parts(value).ok_or(crate::Error::CurieInvalid)?;
        if local.contains(':') || local.starts_with('/') {
            return Err(crate::Error::CurieInvalid);
        }
        Ok(Self(value.into()))
    }
}
impl fmt::Display for Curie {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
crate::text_value!(
    Curie,
    "Curie",
    format!(
        r"^[A-Za-z0-9_.-]+:[^:/{}\x00-\x1f\x7f-\x9f][^:{}\x00-\x1f\x7f-\x9f]*$",
        crate::schema_pattern::WHITESPACE,
        crate::schema_pattern::WHITESPACE
    ),
    "Checked prefix:local ontology identifier."
);
/// A source subject reference, never an embedded variant entity.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "level", rename_all = "snake_case"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
enum Fields {
    Categorical {
        gene: SourceGeneRef,
        category: crate::Category,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        partner: Option<SourceGeneRef>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        region: Option<crate::Region>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        edit: Option<crate::Coding>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        site: Option<crate::ProteinSite>,
        label: Text,
    },
    /// Exact allele identifier or reference-qualified notation.
    Allele(AlleleRef),
    /// Protein-level assertion, without genomic conversion.
    Protein(ProteinRef),
    /// Native ClinVar haplotype and its complete source member identities.
    Haplotype(ClinvarHaplotype),
    /// Native source variant with optional source-stated forms.
    Source(SourceRef),
    /// Compound source profile, without inferred allele identity or phase.
    Profile {
        /// Native profile ID.
        record: Text,
        /// Complete source label.
        label: Text,
        /// Source member references in their native order.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        #[cfg_attr(feature = "schema", schemars(schema_with = "member_refs_schema"))]
        members: Box<[VariantRef]>,
        /// Exact source expression and checked member relations.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        expression: Option<ProfileExpression>,
    },
}

/// Checked source subject reference; at least one source identity or label is required.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Fields"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct VariantRef(Fields);
impl TryFrom<Fields> for VariantRef {
    type Error = crate::Error;
    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        if let Fields::Categorical {
            gene,
            category,
            partner,
            region,
            edit,
            site,
            label,
        } = &fields
        {
            return Self::categorical(
                gene.clone(),
                *category,
                partner.clone(),
                *region,
                *edit,
                site.clone(),
                label.clone(),
            )
            .map_err(|_| crate::Error::VariantRefInvalid);
        }
        match &fields {
            Fields::Haplotype(value) => {
                value.check()?;
                Ok(Self(fields))
            }
            Fields::Allele(AlleleRef { id, hgvs }) if id.is_none() && hgvs.is_none() => {
                Err(crate::Error::VariantRefInvalid)
            }
            Fields::Protein(ProteinRef { hgvs, label, .. })
                if hgvs.is_none() && label.is_none() =>
            {
                Err(crate::Error::VariantRefInvalid)
            }
            Fields::Source(SourceRef { forms, .. })
                if forms
                    .iter()
                    .any(|form| !matches!(form.0, Fields::Allele(_) | Fields::Protein(_))) =>
            {
                Err(crate::Error::VariantRefInvalid)
            }
            Fields::Profile { members, .. }
                if members.iter().any(|m| {
                    matches!(
                        m.0,
                        Fields::Profile { .. } | Fields::Haplotype(_) | Fields::Categorical { .. }
                    )
                }) =>
            {
                Err(crate::Error::VariantRefInvalid)
            }
            Fields::Profile {
                members,
                expression: Some(expression),
                ..
            } => {
                expression.check(members)?;
                Ok(Self(fields))
            }
            _ => Ok(Self(fields)),
        }
    }
}
impl VariantRef {
    /// Preserve a native ClinVar haplotype without deriving an allele or phase.
    pub fn clinvar_haplotype(value: ClinvarHaplotype) -> Result<Self, crate::Error> {
        Fields::Haplotype(value).try_into()
    }
    /// Borrow native ClinVar haplotype grouping and ordered member identities.
    pub fn haplotype(&self) -> Option<&ClinvarHaplotype> {
        match &self.0 {
            Fields::Haplotype(value) => Some(value),
            _ => None,
        }
    }
    /// Borrow the source allele identifier, when present.
    pub fn id(&self) -> Option<&VariantId> {
        match &self.0 {
            Fields::Allele(v) => v.id.as_ref(),
            _ => None,
        }
    }
    /// Borrow the source allele or protein notation, when present.
    pub fn hgvs(&self) -> Option<&HgvsText> {
        match &self.0 {
            Fields::Allele(v) => v.hgvs.as_ref(),
            Fields::Protein(v) => v.hgvs.as_ref(),
            _ => None,
        }
    }
    /// Borrow the protein reference's source gene context.
    pub fn gene(&self) -> Option<&SourceGeneRef> {
        match &self.0 {
            Fields::Protein(v) => Some(&v.gene),
            _ => None,
        }
    }
    /// Borrow the protein or profile label.
    pub fn label(&self) -> Option<&Text> {
        match &self.0 {
            Fields::Protein(v) => v.label.as_ref(),
            Fields::Profile { label, .. } => Some(label),
            _ => None,
        }
    }
    /// Borrow the profile's native record identifier.
    pub fn record(&self) -> Option<&Text> {
        match &self.0 {
            Fields::Profile { record, .. } => Some(record),
            _ => None,
        }
    }
    /// Read profile members in source order as owned references; other levels have none.
    pub fn members(&self) -> impl Iterator<Item = Self> + '_ {
        let members = match &self.0 {
            Fields::Profile { members, .. } => members.as_ref(),
            _ => &[],
        };
        members.iter().cloned()
    }
    /// Borrow a native source variant identifier.
    pub fn source_record(&self) -> Option<&Curie> {
        match &self.0 {
            Fields::Source(SourceRef { record, .. }) => Some(record),
            _ => None,
        }
    }
    /// Borrow exact source feature text without interpreting its biology.
    pub fn feature(&self) -> Option<&Text> {
        match &self.0 {
            Fields::Source(SourceRef { feature, .. }) => Some(feature),
            _ => None,
        }
    }
    /// Read optional source-stated allele or protein forms.
    pub fn forms(&self) -> impl Iterator<Item = &Self> {
        let forms = match &self.0 {
            Fields::Source(SourceRef { forms, .. }) => forms.as_ref(),
            _ => &[],
        };
        forms.iter()
    }
    /// Borrow the source expression and its parsed tree.
    pub fn expression(&self) -> Option<&ProfileExpression> {
        match &self.0 {
            Fields::Profile { expression, .. } => expression.as_ref(),
            _ => None,
        }
    }
    /// Preserve source-stated allele and protein forms without deriving either.
    pub fn source(record: Curie, feature: Text, forms: Box<[Self]>) -> Result<Self, crate::Error> {
        Self::source_with_feature(record, feature, None, forms)
    }
    /// Preserve a source feature identifier when the source supplies one.
    pub fn source_with_feature(
        record: Curie,
        feature: Text,
        feature_record: Option<Curie>,
        forms: Box<[Self]>,
    ) -> Result<Self, crate::Error> {
        Fields::Source(SourceRef {
            record,
            feature,
            feature_record,
            forms,
        })
        .try_into()
    }
    /// Borrow the source-native feature identifier without gene inference.
    pub fn feature_record(&self) -> Option<&Curie> {
        match &self.0 {
            Fields::Source(SourceRef { feature_record, .. }) => feature_record.as_ref(),
            _ => None,
        }
    }
    /// Preserve source profile relations and validate every referenced member.
    pub fn profile_expression(
        record: Text,
        label: Text,
        members: Box<[Self]>,
        expression: ProfileExpression,
    ) -> Result<Self, crate::Error> {
        Fields::Profile {
            record,
            label,
            members,
            expression: Some(expression),
        }
        .try_into()
    }

    pub(crate) fn allele_accession(&self) -> Option<crate::Accession> {
        match &self.0 {
            Fields::Allele(AlleleRef {
                hgvs: Some(hgvs), ..
            }) => hgvs.accession().ok(),
            _ => None,
        }
    }

    /// Reference one checked source allele identifier.
    pub fn allele(id: VariantId) -> Self {
        Self(Fields::Allele(AlleleRef {
            id: Some(id),
            hgvs: None,
        }))
    }
    /// Reference source-qualified notation without computing a genomic allele.
    pub fn allele_hgvs(hgvs: HgvsText) -> Self {
        Self(Fields::Allele(AlleleRef {
            id: None,
            hgvs: Some(hgvs),
        }))
    }
    /// Reference a protein label in its source gene context.
    pub fn protein(gene: SourceGeneRef, label: Text) -> Self {
        Self(Fields::Protein(ProteinRef {
            gene,
            hgvs: None,
            label: Some(label),
        }))
    }
    /// Preserve a source-qualified protein expression and optional source label.
    pub fn protein_hgvs(gene: SourceGeneRef, hgvs: HgvsText, label: Option<Text>) -> Self {
        Self(Fields::Protein(ProteinRef {
            gene,
            hgvs: Some(hgvs),
            label,
        }))
    }
    /// Preserve a compound profile and its native member references.
    pub fn profile(record: Text, label: Text, members: Box<[Self]>) -> Result<Self, crate::Error> {
        Fields::Profile {
            record,
            label,
            members,
            expression: None,
        }
        .try_into()
    }
}
#[cfg(feature = "serde")]
impl Serialize for VariantRef {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(inline))]
struct AlleleRef {
    /// Source identifier.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    id: Option<VariantId>,
    /// Source-qualified HGVS.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    hgvs: Option<HgvsText>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(inline))]
struct ProteinRef {
    /// Source gene.
    gene: SourceGeneRef,
    /// Source protein HGVS.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    hgvs: Option<HgvsText>,
    /// Source protein label.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    label: Option<Text>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(inline))]
struct SourceRef {
    record: Curie,
    feature: Text,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    feature_record: Option<Curie>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    #[cfg_attr(feature = "schema", schemars(schema_with = "source_forms_schema"))]
    forms: Box<[VariantRef]>,
}

// Reuse the canonical payload schemas while restricting each array's admitted levels.
#[cfg(feature = "schema")]
fn tagged_reference<T: schemars::JsonSchema>(
    generator: &mut schemars::SchemaGenerator,
    level: &str,
) -> schemars::Schema {
    let mut schema =
        schemars::_private::json_schema_for_internally_tagged_enum_newtype_variant::<T>(generator);
    schemars::_private::apply_internal_enum_variant_tag(&mut schema, "level", level, false);
    schema
}

#[cfg(feature = "schema")]
fn leaf_refs_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({"oneOf":[
        tagged_reference::<AlleleRef>(generator, "allele"),
        tagged_reference::<ProteinRef>(generator, "protein")
    ]})
}

#[cfg(feature = "schema")]
fn source_forms_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({"type":"array", "items":leaf_refs_schema(generator)})
}

#[cfg(feature = "schema")]
fn member_refs_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({"type":"array", "items":{"oneOf":[
        tagged_reference::<AlleleRef>(generator, "allele"),
        tagged_reference::<ProteinRef>(generator, "protein"),
        tagged_reference::<SourceRef>(generator, "source")
    ]}})
}

impl VariantRef {
    /// Keep a gene-level source category and only its applicable details.
    pub fn categorical(
        gene: SourceGeneRef,
        category: crate::Category,
        partner: Option<SourceGeneRef>,
        region: Option<crate::Region>,
        edit: Option<crate::Coding>,
        site: Option<crate::ProteinSite>,
        label: Text,
    ) -> Result<Self, crate::VariantRefError> {
        if partner.is_some() && category != crate::Category::Fusion {
            return Err(crate::VariantRefError::PartnerMisplaced);
        }
        if (region.is_some() || edit.is_some()) && category != crate::Category::ExonMutation {
            return Err(crate::VariantRefError::RegionMisplaced);
        }
        if edit.is_some_and(|code| !matches!(code, crate::Coding::So(crate::SoCode::Edit(_)))) {
            return Err(crate::VariantRefError::RegionMisplaced);
        }
        if site.is_some() != (category == crate::Category::Residue) {
            return Err(crate::VariantRefError::SiteMisplaced);
        }
        if site.as_ref().is_some_and(|site| {
            SourceGeneRef::try_from(&site.gene).map_or(true, |site_gene| site_gene != gene)
        }) {
            return Err(crate::VariantRefError::SiteGeneMismatch);
        }
        Ok(Self(Fields::Categorical {
            gene,
            category,
            partner,
            region,
            edit,
            site,
            label,
        }))
    }

    /// crate::Category and stated source detail when this is gene-level.
    pub fn category(&self) -> Option<crate::Category> {
        match &self.0 {
            Fields::Categorical { category, .. } => Some(*category),
            _ => None,
        }
    }
}
