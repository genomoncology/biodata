//! Offline release-sized CIViC inputs, deliberately excluded from the routine gate.
#![expect(
    clippy::disallowed_methods,
    reason = "release tests read explicitly configured corpus files"
)]

#[path = "release_corpora/civic.rs"]
mod civic;
#[path = "release_corpora/review.rs"]
mod review;

#[test]
#[ignore = "requires make fetch-corpora; run make release-suite"]
fn civic_monthly_release_has_no_unreviewed_refusals() -> Result<(), Box<dyn std::error::Error>> {
    civic::run()
}
