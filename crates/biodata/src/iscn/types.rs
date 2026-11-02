//! Values hold chromosome structure, never source tokens or byte spans.

/// Ordered clones with their mixture and donor separator.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct Karyotype {
    /// The written mixture marker.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mixture: Option<Mixture>,
    /// The index of the first recipient clone at a written donor separator.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub donor_start: Option<u8>,
    /// The clones in written order.
    pub clones: Vec<Clone>,
}

/// The stated relationship between clones.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Mixture {
    /// A mosaic, written mos.
    Mosaic,
    /// A chimera, written chi.
    Chimera,
}

/// One chromosome constitution and its event list.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct Clone {
    /// The written chromosome count.
    pub count: Count,
    /// The written ploidy annotation, before n.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ploidy: Option<u8>,
    /// The written sex chromosomes; empty with a clone base.
    pub sex: Vec<SexChromosome>,
    /// Whether the sex complement has a constitutional qualifier.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "std::ops::Not::not"))]
    pub sex_constitutional: bool,
    /// An unresolved reference to another clone.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub base: Option<CloneBase>,
    /// The written multiplier of the referenced clone.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub base_times: Option<u8>,
    /// Events in written order.
    pub events: Vec<Event>,
    /// The cells examined, when written.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cells: Option<Cells>,
}

/// An exact, approximate or inclusive chromosome count.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Count {
    /// An exact chromosome count.
    Exact(u8),
    /// An inclusive count range.
    Range(u8, u8),
    /// An approximate count, written with a leading tilde.
    About(u8),
}

/// A chromosome in the sex chromosome constitution.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum SexChromosome {
    /// An X chromosome.
    X,
    /// A Y chromosome.
    Y,
}

/// A reference to an earlier clone.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CloneBase {
    /// The preceding clone.
    Idem,
    /// The stemline.
    Sl(Option<u8>),
    /// A sideline, optionally numbered.
    Sdl(Option<u8>),
}

/// The observed cell count and composite marker.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct Cells {
    /// The number of cells.
    pub count: u32,
    /// Whether the cell count is composite, written cp.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "std::ops::Not::not"))]
    pub composite: bool,
}

/// One chromosome finding with its written modifiers.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct Event {
    /// A gain or loss qualifier on the whole item.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sign: Option<Sign>,
    /// Whether the event is uncertain.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "std::ops::Not::not"))]
    pub uncertain: bool,
    /// Whether the event has the constitutional c qualifier.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "std::ops::Not::not"))]
    pub constitutional: bool,
    /// A count written before an unidentified marker, double minute or ring.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub prefix_count: Option<Count>,
    /// An exact copy multiplier written after x or ×.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub copies: Option<std::num::NonZeroU8>,
    /// The chromosome finding.
    pub item: Item,
}

crate::table::text_table!(Sign, "An explicitly written gain or loss sign.", |text| text, {
    Plus => "+",
    Minus => "-" | "−",
}, "snake_case");

/// The chromosome finding inside an event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Item {
    /// A whole chromosome.
    Whole(Chromosome),
    /// A structural rearrangement.
    Rearrangement(Rearrangement),
    /// A derivative with its ordered structural changes.
    Derivative(Derivative),
    /// An unidentified marker chromosome.
    Marker,
    /// Double minutes.
    Dmin,
    /// A neocentromere with its written path.
    Neo(Neo),
}

/// A structural change over ordered chromosome parts.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct Rearrangement {
    /// The structural family.
    pub kind: RearrangementKind,
    /// The written pseudo qualifier.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "std::ops::Not::not"))]
    pub pseudo: bool,
    /// A ring qualifier on a dicentric or tricentric structure.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "std::ops::Not::not"))]
    pub ring: bool,
    /// Whether this structural change is uncertain.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "std::ops::Not::not"))]
    pub uncertain: bool,
    /// Chromosomes and their corresponding breakpoint groups.
    pub parts: Vec<Part>,
}

crate::table::text_table!(RearrangementKind, "The structural change named by an ISCN marker.", |text| text, {
    Tricentric => "trc",
    Triplication => "trp",
    Quadruplication => "qdp",
    Pulverization => "pvz",
    HomogeneouslyStainingRegion => "hsr",
    FragileSite => "fra",
    Fission => "fis",
    ChromatidBreak => "chtb",
    ChromatidExchange => "chte",
    ChromatidGap => "chtg",
    Dicentric => "dic",
    Isodicentric => "idic",
    Robertsonian => "rob",
    Ring => "r",
    Translocation => "t",
    Deletion => "del",
    Duplication => "dup",
    Inversion => "inv",
    Insertion => "ins",
    Isochromosome => "i",
    AdditionalMaterial => "add",
}, "snake_case");

/// One chromosome and its ordered breakpoints.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct Part {
    /// The chromosome involved.
    pub chromosome: Chromosome,
    /// Its breakpoints in written order.
    pub breakpoints: Vec<Breakpoint>,
}

/// An autosome, sex chromosome or unknown chromosome.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Chromosome {
    /// An autosome numbered 1 through 22.
    Autosome(u8),
    /// The X chromosome.
    X,
    /// The Y chromosome.
    Y,
    /// An unknown chromosome, written ?.
    Unknown,
}

/// A site with uncertainty and an optional alternative.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct Breakpoint {
    /// The site of the breakpoint.
    pub site: Site,
    /// An alternative site or a range of sites.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub alternative: Option<Alternative>,
}

/// A chromosome site and its stated uncertainty.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Site {
    /// The chromosome location.
    pub location: SiteKind,
    /// Whether a question mark qualifies this location.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "std::ops::Not::not"))]
    pub uncertain: bool,
}

/// A chromosome band, arm, terminal, centromere or unknown location.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum SiteKind {
    /// An unknown breakpoint, written ?.
    Unknown,
    /// A numbered band on an arm.
    Band(Arm, Band),
    /// An arm with no known band.
    Arm(Arm),
    /// The terminal end of an arm.
    Terminal(Arm),
    /// The centromere, written cen.
    Centromere,
}

crate::table::text_table!(Arm, "The short or long chromosome arm.", |text| text, {
    P => "p",
    Q => "q",
}, "snake_case");

/// A numbered chromosome band, with an optional one or two digit sub-band.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Band {
    region: u8,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    band: Option<u8>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    sub: Option<u8>,
}

impl Band {
    /// Constructs a band. Region is 1 through 9; band is unknown or 0 through 9;
    /// sub-band is 1 through 99 with no leading zero in its canonical spelling.
    pub fn new(region: u8, band: Option<u8>, sub: Option<u8>) -> Option<Self> {
        (region > 0
            && region <= 9
            && band.is_none_or(|n| n <= 9)
            && sub.is_none_or(|n| n > 0 && n <= 99))
        .then_some(Self { region, band, sub })
    }

    /// The first digit of the band.
    pub fn region(self) -> u8 {
        self.region
    }
    /// The second digit, absent when the band digit is unknown.
    pub fn band(self) -> Option<u8> {
        self.band
    }
    /// The optional sub-band number.
    pub fn sub(self) -> Option<u8> {
        self.sub
    }
}

/// A derivative and the rearrangements written after it.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct Derivative {
    /// Whether the derivative is an isoderivative, ider.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "std::ops::Not::not"))]
    pub isoderivative: bool,
    /// The chromosomes and optional breakpoints identifying the derivative.
    pub parts: Vec<Part>,
    /// Structural changes, in written order.
    pub changes: Vec<Rearrangement>,
}

/// An explicitly written alternative breakpoint.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Alternative {
    /// Either site may apply.
    Or(Site),
    /// The site lies within the written endpoints.
    Range(Site),
}

/// A neocentromere and its written path endpoints.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct Neo {
    /// The chromosome bearing the path.
    pub chromosome: Chromosome,
    /// The path's start.
    pub start: Site,
    /// The path's end, followed by a colon in this notation.
    pub end: Site,
}
