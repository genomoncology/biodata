//! The Cellosaurus XSD defines these exact sex labels.
// https://ftp.expasy.org/databases/cellosaurus/cellosaurus.xsd
crate::native_enum! {
    /// Native Cellosaurus sex label, with new publisher values retained.
    CellLineSex {
        /// Publisher Female label.
        Female => "Female",
        /// Publisher Male label.
        Male => "Male",
        /// Publisher Mixed sex label.
        Mixed => "Mixed sex",
        /// Publisher Sex ambiguous label.
        Ambiguous => "Sex ambiguous",
        /// Publisher Sex unspecified label.
        Unspecified => "Sex unspecified",
    }
}
