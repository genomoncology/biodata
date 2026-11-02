//! Publisher age codes for a FAERS patient onset interval.
crate::wire_enum! {
/// Exact openFDA patientonsetageunit codes.
FaersAgeUnit {
    /// Decade.
    Decade => "800",
    /// Year.
    Year => "801",
    /// Month.
    Month => "802",
    /// Week.
    Week => "803",
    /// Day.
    Day => "804",
    /// Hour.
    Hour => "805",
}
}
