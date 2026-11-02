//! Coded [HPO mode-of-inheritance terms](https://github.com/obophenotype/human-phenotype-ontology/blob/master/hp.obo),
//! without source interpretation. The root term records no specific inheritance mode.

/// One supported Human Phenotype Ontology mode-of-inheritance term.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModeOfInheritance(&'static str);

impl ModeOfInheritance {
    /// Autosomal dominant inheritance, HP:0000006.
    pub const AUTOSOMAL_DOMINANT: Self = Self("HP:0000006");
    /// Autosomal recessive inheritance, HP:0000007.
    pub const AUTOSOMAL_RECESSIVE: Self = Self("HP:0000007");
    /// Mitochondrial inheritance, HP:0001427.
    pub const MITOCHONDRIAL: Self = Self("HP:0001427");
    /// Semidominant inheritance, HP:0032113.
    pub const SEMIDOMINANT: Self = Self("HP:0032113");
    /// X-linked inheritance, HP:0001417.
    pub const X_LINKED: Self = Self("HP:0001417");
    /// The HPO category root, HP:0000005. A source can use it for an
    /// undetermined assertion, but the code alone does not say that.
    pub const MODE_OF_INHERITANCE: Self = Self("HP:0000005");

    /// Parse one supported HPO term code.
    pub fn from_hpo_code(code: &str) -> Option<Self> {
        match code {
            "HP:0000006" => Some(Self::AUTOSOMAL_DOMINANT),
            "HP:0000007" => Some(Self::AUTOSOMAL_RECESSIVE),
            "HP:0001427" => Some(Self::MITOCHONDRIAL),
            "HP:0032113" => Some(Self::SEMIDOMINANT),
            "HP:0001417" => Some(Self::X_LINKED),
            "HP:0000005" => Some(Self::MODE_OF_INHERITANCE),
            _ => None,
        }
    }

    /// Human Phenotype Ontology term code.
    pub const fn hpo_code(self) -> &'static str {
        self.0
    }
}
