# 0024 — protein-range-identity

Status: COMPLETE.
Landed: ad4daa1933c8cfdc95f30016593625b20a50ec84

Milestone: 1

## Outcome

A protein identity over a range is a value. `hgvs::parse` accepts `p.Ala2_Gly3=`, the predicted `p.(Ala2_Gly3=)`, and the one-letter `p.A2_G3=`, and formats each back to the standard's three-letter text, which parses to the same value. `ProteinChange::Same` holds `Option<ProteinRange>`, the location type deletions and duplications already use, so one variant says "no change" at a site, over a range, or across the protein. `ProteinRange::new(first, last)` builds a range and refuses a last position that does not lie after the first with `ProteinRangeError` (code `hgvs.protein_range.order`). The parser checks range order through `ProteinRange::new`, and one shared rule refuses a stop at the start of a range for deletions, duplications, deletion-insertions, insertions and range identities. Equal positions are refused, not folded: one site is written `ProteinRange::from(site)`, as the parser requires `p.Ala2=` rather than `p.Ala2_Ala2=`. The refusal code `hgvs.protein_range_equality.unsupported` is removed, because nothing raises it.

## Why

The HGVS recommendations define `=` as "a sequence was tested but found unchanged" ([general recommendations](https://hgvs-nomenclature.org/stable/recommendations/general/)) and give its syntax as `position_or_range "="` ([summary](https://hgvs-nomenclature.org/stable/recommendations/summary/)), with the range example `g.4567_4569=` ([DNA substitution](https://hgvs-nomenclature.org/stable/recommendations/DNA/substitution/)). The protein pages show only single residues (`p.Cys188=`, `p.(Lys874=)`) and `p.=`, so the protein range form follows the general rule by analogy rather than from a protein example. BioData already parses the nucleotide range identity `g.21_22=`, and refusing the protein form left a stated "no change" over a protein range with no value.

## Proof

- Proof: `crates/biodata/tests/cases/public/hgvs-protein.jsonl` replaces the pinned refusal of `p.Ala2_Gly3=` with parsed rows that state the serialized value and the round trip, and keeps near-miss refusals: reversed `p.Gly3_Ala2=`, equal `p.Ala2_Ala2=`, a trailing edit `p.Ala2_Gly3=Ser` or `p.Ala2_Gly3Ser`, malformed `p.Ala2_=`, `p.Ala2_Gly=` and `p.(Ala2_Gly3=`, and a range identity starting at a stop, `p.Ter110_Gly112=`, which a deletion's range also refuses. A range ending at a stop (`p.Leu109_Ter110=`) and `Xaa` at either end of a range parse, for identity as for deletion and duplication. `alteration-protein.jsonl` adds `ABL1 A7_G8=`. `tests/protein_identity.rs` checks the constructor, its refusals, and that a constructed identity formats to the standard text and parses back equal. The parsed rows fail on the 1.0.0 tag. One `pm job` runs `make gate` and passes.

## Starts from

- Starts from: `edit` and `ProteinChange` in `crates/biodata/src/hgvs/protein.rs`; `ProteinRange` and its `Display`, which deletions already use; the nucleotide range identity in `crates/biodata/src/hgvs/nucleotide.rs`.
- Ian, 2026-11-11: BioData is "not officially 1.0 yet, so make whatever changes necessary." The [no backwards compatibility ruling](../decisions/2026-10-31-biodata-makes-no-backwards-compatibility-promises.md) keeps no alias for a removed code.

## Keeps

- Keeps: the size and alignment of `ProteinSite`, `ProteinRange`, `ProteinChange`, `hgvs::Change`, `Hgvs`, `alteration::Change` and `Alteration`, measured on the base commit and pinned by `crates/biodata/tests/hgvs_layout.rs`. Every input that parsed before parses to the same value and serializes to the same bytes in every serde format: a one-site identity still serializes as the bare site (`{"same":{"residue":"Leu","position":858}}`) and `p.=` as `{"same":null}`; these rows now state their values. A range identity serializes as `{"same":{"start":…,"end":…}}`, the shape a deletion's range has. HGVS change values have no JSON schema, so no schema changes. `HgvsText` does not run the parser and is unchanged.

## Defers

- Defers: making `ProteinRange`'s fields private. They are public today and deletions, duplications and deletion-insertions build them directly, so a hand-built range can still break the order the parser and `ProteinRange::new` enforce. Closing that changes every protein edit's construction and belongs in its own ticket.

## Release note

- Release note: for whoever cuts the next registry release, this branch changes the public interface since tag v1.0.0 (`ProteinChange::Same` holds an `Option<ProteinRange>` location instead of an `Option<ProteinSite>`, and `ProteinCode::RangeEqualityUnsupported` is gone), while `Cargo.toml` still says version 1.0.0. Choose the next version for that break; this ticket does not change it.

## Changes

- Changes: `crates/biodata/src/hgvs/`, `crates/biodata/src/peptide.rs`, `crates/biodata/tests/`, `sdlc/designs/2026-10-28-reconciliation.md`
