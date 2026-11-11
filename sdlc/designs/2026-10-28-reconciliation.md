# Reconciliation of the two BioData crates

This record gives every public item of BioMCP's `biodata` crate and the second consumer's `biodata` crate one merge rule. The import ticket merges the crates by these rows. The record states the commits it read, and it is refreshed when BioMCP's PM names the extraction commit.

BioMCP commit: `bc6ca40de65c02c3e9aecce65b4a7294aa334727`
Second consumer commit: `e7da7041a6e83b557f9a9e6b3065ae0a66bc44b0`

Both commits were read at `crates/biodata`. File paths in the rows are relative to that folder.

## Check

`scripts/check-reconciliation.py BIOMCP_REPO SECOND_REPO` takes a local clone of each consumer. It builds each crate at its recorded commit with rustdoc, lists every public item, and fails when an item lacks exactly one row with one rule. Its docstring says what counts as an item and every way it fails. Run it after every refresh.

## Rules

| Rule | Meaning |
|---|---|
| MOVE | Only one crate has the item. It moves unchanged with its tests. A BioMCP item keeps its serde and schema impls, behind the features the serialization decision sets. A second consumer item that a schema addition reaches gains a schema impl behind the `schema` feature. |
| ONE | Both crates have one meaning for the item. The second consumer's item is the base and keeps its layout, parse behavior and serde. The merge adds BioMCP's schema impl behind the `schema` feature and any trait impl BioMCP needs. BioMCP's wire bytes do not change. |
| ONE-W | Both crates have one meaning for the item, and the second consumer's item is wider than BioMCP's wire. BioData holds one type that writes BioMCP's wire and keeps BioMCP's Rust API. The second consumer's wider fields, enum cases and coding systems join as optional additions, omitted when absent, and the second consumer adapts its code. The schema additions table lists each addition. |
| NAME | The items share a name but differ in meaning, or stay apart until their meaning is ruled. Both stay, and the note gives the new name. |

These rules apply to every row:

- Where both crates name one meaning differently, the second consumer's name wins and BioMCP's code renames. A BioMCP type that renames keeps its schema name.
- A ONE item returns the second consumer's error type. Each second consumer error type gains a `code()` method. The merged `Error` keeps BioMCP's code strings. No conversion joins the two. `IdError.code()` is `id.value.invalid`, because the compact shared error carries no namespace-specific variant. Its existing display text stays unchanged.
- The second consumer's private test cases and evidence (`tests/cases/private`, `tests/evidence/private` and `tests/private`) do not enter this public repository. The second consumer keeps them and runs them against the merged crate.
- The merged crate keeps one source file per value. A ONE item lives in the second consumer's file.

## Hot-path changes

The second consumer's hot-path values are the base. The target is no change. The table lists every change the rows make to them. The second consumer measures each change in its own repository with its output and index byte comparison, its lookup timing and the `catalog_speed` measurement in its private tests.

| Item | Change | Effect on layout, parse and cost |
|---|---|---|
| `Accession` in `src/value.rs` | Adds BioMCP's `schemars::JsonSchema` impl. | None. The impl builds only with `schema`, so the second consumer's build, which turns `schema` off, compiles none of it. |
| `Base` in `src/value.rs` | Adds BioMCP's schema impl. | None. As for `Accession`. |
| `Chromosome` in `src/reference.rs` | Adds BioMCP's schema impl. | None. As for `Accession`. |
| `parse_chromosome` in `src/reference.rs` | None. `ReferenceError` already has `code()`. | None. |
| `Position` in `src/hgvs/nucleotide.rs` | None. | None. |
| `AlleleKey` in `src/coordinate.rs` | None. `CoordinateError` already has `code()`. | None. |
| `Span` in `src/outcome.rs` | None. | None. |
| The HGVS parser in `src/hgvs/` | None. Two of its functions exceed BioMCP's line limit and get named lint exceptions instead of a split (see Lints below). | None. |
| `ProteinChange` in `src/hgvs/protein.rs` | Ticket 0024: `Same` holds `Option<ProteinRange>` instead of `Option<ProteinSite>`, so a protein identity can cover a range. | None measured. Sizes and alignments of `ProteinSite`, `ProteinRange`, `ProteinChange`, `hgvs::Change`, `Hgvs`, `alteration::Change` and `Alteration` stay 8/4, 16/4, 48/8, 96/8, 144/8, 144/8 and 168/8 bytes on 64-bit targets, pinned by `tests/hgvs_layout.rs`. Every input that parsed before parses the same way and serializes to the same bytes in every serde format; the parser drops one early refusal check. Code that builds or matches `Same(Some(site))` adapts to `Same(Some(site.into()))` and reads `range.start`. |

## Known conflicts

| Conflict | Finding at the recorded commits | Rule |
|---|---|---|
| Variant ID spelling | Both crates write published text such as `rs123`. | ONE. |
| Ensembl gene ID | BioMCP's `GeneRef` uses `EnsemblGeneId`, which keeps an optional version. The second consumer's `GeneRef` uses an unversioned `AccessionBase`. | NAME. Two gene reference types stay; the `GeneRef` row says how. |
| Population frequency fields | BioMCP now has every field the second consumer has. BioMCP's fractions are `Fraction` and its fields are private; the second consumer's fractions are `f64` and its fields are public. | ONE-W, with no schema additions. |
| Coding systems | Both write `{system, code}` with the same spellings for BioMCP's systems. The second consumer adds five more. The two shared ClinVar systems now exist in BioMCP too. | ONE-W. `Coding` takes the union of systems, listed in the schema additions table. |
| Tags on the HGVS change enums | The second consumer's HGVS enums serialize externally tagged in snake case, with no deserializer and no schema. BioMCP's wire enums are internally tagged with `deny_unknown_fields` and a schema. BioMCP has no HGVS values today. | MOVE, with the tags unchanged. |

## Rulings and open points

The import ticket settles the required subject and protein-site schema questions:

1. `Statement.subject` becomes `Subject`. `From<VariantRef> for Subject` preserves every existing variant subject's JSON bytes. BioMCP adapts its Rust field construction.
2. The second consumer's `GeneRef` gains schema name `SymbolGeneRef`. `SourceGeneRef` retains schema name `GeneRef`. Categorical site validation converts `site.gene` into `SourceGeneRef` before comparing it with the case's gene.

## Extraction drift

The frozen extraction commit is the new baseline for every existing BioMCP case, field and schema. Differences between it and the earlier reconciliation source do not authorize another change on import. Existing cases remain byte-identical. The refreshed inventory adds the source haplotype, source reference, profile expression, coding notation, identifier, upload time and publisher coding values and removes the error types now folded into `Error`.

The PM settled both new conflicts in the import ticket:

- `Curie` adopts BioMCP's frozen grammar. It refuses a local part containing `:` or beginning `/`. This narrows the second consumer's parser and keeps the published schema exact.
- `ClinvarReviewStatusCode` merges into `ReviewStatus`; `ClinvarAlleleOriginCode` merges into `AlleleOrigin`. Both keep the second consumer's publisher-label `FromStr` and keep BioMCP's schema names. Their serialization keeps every BioMCP case byte-identical. BioMCP's switch adapts its construction.

## Schema additions

Each ONE-W row adds these to BioMCP's `biodata.*` schemas, in every schema that holds the named `$defs` key or root. Every addition is optional and omitted when absent, except where the last column says otherwise. A new `$defs` key comes from the second consumer's item of that name, which moves under its MOVE row, together with the second consumer's types that item holds. A gene in an addition uses `SourceGeneRef` (schema name `GeneRef`), which holds every value of the second consumer's `GeneRef`.

| Type | Addition | Kind | New `$defs` | Required |
|---|---|---|---|---|
| `Attribution` | `approved` | field, `PartialDate` | none | no |
| `Attribution` | `published` | field, `PartialDate` | none | no |
| `Proposition` | `gene_disease_validity`, `haploinsufficiency`, `triplosensitivity`, `gene_role` | enum cases | none | no |
| `Statement` | `method` | field | `Method` | no |
| `Statement` | `mode_of_inheritance` | field, `Term` | none | no |
| `Statement` | `criteria_not_met` | field, list of `Text`, omitted when empty | none | no |
| `Statement` | `status` | field, `Term` | none | no |
| `Statement` | `subject` with level `gene` or `region` | cases of a required field | the gene and region subject shapes | yes, within added subject cases only |
| `VariantRef` | level `categorical` | case | `Category` (`amplification`, `deletion`, `fusion`, `exon_mutation`, `oncogenic_mutation`, `residue`) | no, as a case |
| `VariantRef` | `categorical.gene`, `categorical.category`, `categorical.label` | fields inside the new case | none | yes, inside the new case only |
| `VariantRef` | `categorical.partner`, `categorical.region`, `categorical.edit`, `categorical.site` | fields inside the new case | `Region`, `ProteinSite` | no; `ProteinSite.gene` uses `SymbolGeneRef` |
| `PredictorScore` | `gene` | field, `SourceGeneRef` | none | no |
| `PredictorScore` | `transcript` | field | `TranscriptId` | no |
| `PopulationFrequency` | none | | | |
| `Coding` | systems `clinvar_classification_type`, `so`, `hpo`, `clingen_gene_validity_2017`, `clingen_dosage_score` | `oneOf` cases | `ClinvarClassificationCode`, `SoCode`, `HpoInheritanceCode`, `ClingenGeneValidity2017Code`, `ClingenDosageScoreCode` | no |
| `SequenceLocation` | none | | | |
| `GeneSymbol` | none | | | |

### Shared schema change map

The source baseline is the raw `JsonSchema` output at the frozen commit, before a consumer changes it for replies. All shared types that implement that trait retain their root schema name. `RefSeqPrefix`, `EnsemblFeature`, `LrgPart` and `Error` have no standalone schema in that baseline.

| Raw schema | Changes |
|---|---|
| `Attribution` | Optional `approved` and `published` properties. |
| `Proposition` | Cases `gene_disease_validity`, `haploinsufficiency`, `triplosensitivity`, `gene_role`. |
| `Coding` | Five systems and their code definitions, as the additions table lists. Existing cases stay intact. |
| `VariantRef` | A categorical case and its reachable definitions, as the additions table lists. |
| `PredictorScore` | Optional `gene` and `transcript`, plus reachable additions. |
| `Statement` | Optional `method`, `mode_of_inheritance`, `criteria_not_met`, `status`; `subject.$ref` changes from `VariantRef` to `Subject`. `Subject` adds gene and region cases while preserving the existing variant branch. |
| `PopulationFrequency`, `Term`, `UniProtFeature` | No direct field change. Reachable definitions acquire the additions above. |
| `Variant` | No direct field change. Its `VariantId` definition acquires the exact bound narrowing below. |
| `OmimId` | Six arbitrary digits narrow to `[1-6]` followed by five digits, matching the checked constructor range. |
| `ClinVarVariationId`, `ClinVarAlleleId` | Positive decimal bound changes from `u64` to `u32`; their frozen prefixes are unchanged. |
| `VcvId`, `RcvId`, `ScvId` | Optional positive decimal version bound changes from arbitrary length to `u32`. |
| `CosmicId` | Positive decimal suffix bound changes from arbitrary length to `u64`. |
| `VariantId` | Its embedded variation and Cosmic alternatives acquire those bounds; all seven alternatives remain in their frozen order. |
| `OmimId` | The parser and constructor narrow to 100000 through 699999; the schema becomes `[1-6][0-9]{5}`. |

The added subject definitions are `Subject`, `GeneSubjectWire`, `GeneLevel`, `RegionSubjectWire` and `RegionLevel`. Added category details reach `Category`, `ProteinSite`, `SymbolGeneRef`, `Region`, `RegionKind`, `TranscriptReference`, and their existing scalar dependencies. `PredictorScore.transcript` adds `TranscriptId`. `Method` and the five added coding payloads have their own definitions. A new `SymbolGeneRef` states an unversioned `ENSG` gene ID and the checked `GeneSymbol` label, so it does not claim arbitrary accession families.

No existing enum case, required property or definition is removed. The required subject reference change above is explicit. Every other observed difference is an optional addition, a newly reachable definition, or one of the eight named pattern changes. The remaining shared schemas retain their frozen shape.

### Affected published schema IDs

The table names every frozen `biodata.*` schema that directly describes or holds a changed raw schema. The changes in each cell follow the raw schema table above, including changes in a reachable definition.

| Published schema ID | Changed raw schemas it holds |
|---|---|
| `biodata.authorization-event/4` | `Attribution` |
| `biodata.clinvar-allele-id/2` | `ClinVarAlleleId` |
| `biodata.clinvar-variation-id/2` | `ClinVarVariationId` |
| `biodata.cosmic-id/1` | `CosmicId` |
| `biodata.disease-association/7` | `Attribution` |
| `biodata.disease-relation/4` | `Attribution` |
| `biodata.disease-statistic/4` | `Attribution` |
| `biodata.disease-synonym/3` | `Attribution` |
| `biodata.drug-identity-relation/5` | `Attribution` |
| `biodata.drug-indication-claim/7` | `Attribution` |
| `biodata.drug-interaction-claim/7` | `Attribution` |
| `biodata.drug-label-reference/4` | `Attribution` |
| `biodata.drug-label-section/4` | `Attribution` |
| `biodata.drug-product/5` | `Attribution` |
| `biodata.drug-target-relation/7` | `Attribution` |
| `biodata.gene/7` | `OmimId` |
| `biodata.omim-id/1` | `OmimId` |
| `biodata.population-frequency/8` | `Attribution`, `ClinVarAlleleId`, `ClinVarVariationId`, `PopulationFrequency`, `VariantId`, `VariantRef`, `VcvId` |
| `biodata.rcv-id/2` | `RcvId` |
| `biodata.scv-id/2` | `ScvId` |
| `biodata.statement/7` | `Attribution`, `ClinVarAlleleId`, `ClinVarVariationId`, `Coding`, `Proposition`, `Statement`, `Term`, `VariantId`, `VariantRef`, `VcvId` |
| `biodata.variant-id/3` | `VariantId` |
| `biodata.variant/7` | `Variant`, `VariantId` |
| `biodata.vcv-id/2` | `VcvId` |

## Changes to the second consumer's parsing

BioData's own types hold BioMCP's limits. These narrow what the second consumer's parsers accept. None is a hot-path value. The second consumer's PM confirms each.

| Type | Change |
|---|---|
| `GeneSymbol` in `src/value.rs` | Refuses symbols over 255 bytes. |
| `OmimId` in `src/identifier.rs` | Constructor and parser accept 100000 through 699999. |
| `Curie` in `src/variant/statement/reference.rs` | Refuses local parts containing `:` or beginning `/`. |
| `Text` in `src/source_text.rs` | Refuses unsafe control characters and text beyond the frozen bound; `from_source` visibly escapes controls. |
| `HgvsText` in `src/source_text.rs` | Refuses spaces, nonnumeric bodies outside the explicit positionless forms, and nonprotein bodies on protein references. |
| `Expression` in `src/variant/notation.rs` | Applies the same body check to chromosome expressions. |
| `VcvId`, `RcvId` and `ScvId` in `src/identifier.rs` | Refuse ten-digit numbers. |

One later change widens the second consumer's parsing instead. Ticket 0024 makes a protein identity over a range, such as `p.Ala2_Gly3=`, parse through `hgvs::parse` and `alteration::parse` as a `ProteinChange::Same` value, where both parsers refused it with `hgvs.protein_range_equality.unsupported`. That code is removed because nothing raises it. A range identity that starts at a stop is refused with `hgvs.protein_edit.invalid`, as a deletion's range is.

## Other compatibility effects at extraction

- `PartialDate` keeps the frozen schema. Its parser additionally rejects calendar-impossible days, including `2023-02-29` and `2024-02-30`; this is an explicit schema/parser subset exception.

- `PopulationFrequency` keeps BioMCP's validator. Mitochondrial-field placement checks only the explicitly stated build, rather than searching both builds as the second consumer did.
- Existing shared record schemas keep BioMCP's openness to extra fields. The second consumer's `deny_unknown_fields` does not narrow those schemas or their matching parsers.
- `AccessionBase` retains compact parsed storage. An LRG spelling with redundant zeros, such as `LRG_0001t01`, serializes canonically as `LRG_1t1`. Fixed-width RefSeq and Ensembl digits retain their zeros. This is the PM's explicit wire exception.
- `VcvId`, `RcvId`, `ScvId` retain optional `NonZeroU32` versions and version-sensitive equality and hashing. `same_accession` compares only the accession number for consumers that need that identity. Each version pattern narrows from arbitrary positive decimal to the exact `u32` bound.
- `CosmicId` retains `NonZeroU64`; its suffix pattern narrows from arbitrary positive decimal to the exact `u64` bound.
- `ClinVarVariationId` and `ClinVarAlleleId` retain `NonZeroU32`; their patterns narrow from the frozen `u64` bound to `u32`. `VariantId` incorporates the narrower variation and Cosmic patterns. These are disclosed acceptance deltas, not optional additions.
- `OmimId` additionally narrows to the constructor range 100000 through 699999; the schema is `[1-6]` followed by five digits. This is the eighth pattern narrowing. No remaining pattern changes are intended. Each narrowed numeric pattern uses the same width bound as its parser.

## Frozen schema exceptions

The PM's 1.0 ruling preserves the frozen schema shapes. Exact scalar and field-shape properties remain in `tests/schema_contract.rs`. Existing relational, resource and canonical-construction rules remain parser-only by class. The table records the known boundaries, including the scalar gaps explicitly retained for 1.0. `tests/schema_exceptions.rs` exercises each independent rule through the public deserializer and proves that the frozen schema accepts its rejected witness. The existing calendar witness stays in `tests/schema_contract.rs`. No exception permits changing a parser or weakening the scalar properties.

| Value | Parser-only rules | Boundary witnesses |
|---|---|---|
| `Span0` | Start does not follow end. | `span_order` |
| `Bound` | Stated limits are ordered; at least one limit is nonnull; limits fit their integer storage. The last two are retained scalar gaps in the frozen shape. | `bound_order`, `bound_needs_a_nonnull_limit`, `bound_integer_storage_limit` |
| `UncertainSpan0` | The start and end ranges are compatible. | `uncertain_span_order` |
| `Allele` | The reference length matches the location; at least one allele side is nonempty. | `allele_reference_length`, `allele_needs_a_nonempty_side` |
| `Expression` | The value follows a supported form and agrees with its declared syntax. | `expression_requires_a_supported_form`, `expression_syntax_matches_value` |
| `PartialDate` | The day exists in the stated calendar month and year. | `partial_date_schema_differs_only_for_impossible_calendar_days` |
| `Mane` | Inline accession patterns omit the version storage bound and canonical RefSeq width check. These are retained scalar gaps. | `mane_version_storage_limit`, `mane_refseq_canonical_width` |
| `Accession`, `AccessionBase`, `HgvsText` | Parsing retains the input resource budget even though the frozen LRG pattern admits unbounded redundant zeros. | `accession_resource_budget` |
| `VariantRef` leaf | An allele or protein leaf has at least one source identity or label. | `variant_leaf_needs_an_identity` |
| `VariantRef` haplotype | The accession is unversioned, the member list is nonempty, and the separately stated version forms a valid accession. | `haplotype_accession_is_unversioned`, `haplotype_has_members`, `haplotype_has_a_parseable_separate_version` |
| `ProfileExpression` | Source text follows its grammar, its parsed tree agrees with the supplied tree, and parsing retains depth and node budgets. The grammar also constrains Boolean operands and negation. | `profile_source_grammar`, `profile_source_tree_agreement`, `profile_expression_depth_budget`, `profile_expression_node_budget` |
| `VariantRef` profile with an expression | Members carry source records, those records are unique, and they cover every expression reference. Source-form and profile-member level restrictions remain expressed in the schemas. | `profile_members_have_source_records`, `profile_member_records_are_unique`, `profile_members_cover_expression_references` |
| `PopulationFrequency` | Allele count does not exceed its number; ancestry and group-maximum labels are exclusive; mitochondrial fields agree with the stated accession and build. | `population_count_does_not_exceed_number`, `population_group_labels_are_exclusive`, `population_fields_match_the_stated_accession_build` |
| `VariantRef` categorical | A partner requires fusion; region or edit requires exon mutation; edit uses the SO edit vocabulary; a site appears exactly for residue and names the same gene. | `categorical_partner_requires_fusion`, `categorical_region_or_edit_requires_exon_mutation`, `categorical_edit_uses_the_edit_vocabulary`, `categorical_site_is_present_exactly_for_residue`, `categorical_site_names_the_same_gene` |
| `ProteinSite` | The last position does not precede the first. | `protein_site_order` |
| `Subject` region | The interval states a build. | `region_subject_needs_build` |

These rules propagate into every containing record. Plain records and closed vocabularies add no independent parser-only rule. A post-1.0 follow-up reviews the retained scalar gaps and how future schema versions state parser-only constraints; it does not change the 1.0 shapes during this import.

## Architecture proposals

The import ticket holds the PM's rulings on these proposals.

### Crate layout

Options: one crate with features; a small workspace that splits parsers from entity values. One crate keeps one version, one pin for BioMCP and one lint set, and both consumers already use one crate. A workspace isolates compile time but adds versions to keep in step. Recommendation: one crate named `biodata`, with modules from both consumers and the features below.

### Serialization

Options: serde always on, as BioMCP has it; serde behind a feature, as the second consumer has it. Recommendation: a `serde` feature, off by default, and a `schema` feature that turns on `serde` and adds `schemars`. BioMCP turns on `schema`. BioMCP's derives move to `cfg_attr(feature = "serde", ...)`, which changes no bytes when the feature is on. No `schemars` derive or impl builds without `schema`. The hot-path values compile as they do today when the features are off.

### Lints

Recommendation: the union of both lint sets, at the stricter level of each. Rust: `unsafe_code`, `unreachable_pub` and `missing_debug_implementations` forbidden, as the second consumer has them, and `missing_docs` denied. Clippy: both deny lists, which adds `too_many_lines`, `disallowed_methods` and `disallowed_types` to the second consumer's set. `clippy.toml` takes the union of both disallowed lists and BioMCP's line threshold of 80. At the recorded commits, BioMCP's crate passes the union with no change. The second consumer's crate fails it only on `too_many_lines`, in these functions, which get `#[expect(clippy::too_many_lines, reason = ...)]`:

| Function | File |
|---|---|
| `unsupported` | `src/alteration/deferred.rs` |
| `body` | `src/hgvs/nucleotide.rs` |
| `edit` | `src/hgvs/protein.rs` |
| `difference` | `src/peptide.rs` |
| `transcript_checks_coding_bounds_and_gene_namespace` | `tests/transcript_model.rs` |

Splitting the two HGVS functions instead would touch the hot path, so the exceptions stay until a measured ticket splits them.

### Schema ownership

BioMCP moves the schema IDs it owns to `biomcp.*` before extraction, so BioData owns every `biodata.*` ID it inherits.

### Distribution

Options: a Git dependency pinned to a commit; a vendored copy in BioMCP's offline build; a crates.io release, which needs Ian's approval. Recommendation: BioMCP pins `biodata` by Git tag and exact commit. Its offline release build vendors the dependency with `cargo vendor`, so the pin resolves on clean hosted runners and offline. A crates.io release waits for Ian.

### Rules carried from the old repository

Recommendation: carry forward three rules that both crates already follow. Values are parsed without being interpreted. Parsers return typed refusals with stable codes and keep no rejected input. Fields stay private behind validating constructors, except plain records whose fields carry no invariant.

### Python distribution name

This waits for the SDK milestone, as the plan says. No proposal yet.

## Items
### Items in both crates

| BioMCP | Second consumer | Rule | Note | Merged public item |
|---|---|---|---|---|
| `Accession` in `src/variant/accession.rs` | `Accession` in `src/value.rs` | ONE | Hot path. BioMCP matches on public variants today; it moves to `refseq_parts`, `ensembl_parts`, `version` and the checked constructors. Parse error becomes `Refusal`; the code strings already match. | `Accession` in `src/value.rs` |
| `AccessionBase` in `src/variant/accession.rs` | `AccessionBase` in `src/value.rs` | ONE | Storage changes from text to a `Copy` accession; the text pattern is the same. | `AccessionBase` in `src/value.rs` |
| `RefSeqPrefix` in `src/variant/accession.rs` | `RefSeqPrefix` in `src/value.rs` | ONE | Never serialized alone in BioMCP; it reaches the wire only inside `Accession` text. | `RefSeqPrefix` in `src/value.rs` |
| `EnsemblFeature` in `src/variant/accession.rs` | `EnsemblFeature` in `src/value.rs` | ONE | As `RefSeqPrefix`. | `EnsemblFeature` in `src/value.rs` |
| `LrgPart` in `src/variant/accession.rs` | `LrgPart` in `src/value.rs` | ONE | Same declaration. | `LrgPart` in `src/value.rs` |
| `Base` in `src/variant/sequence.rs` | `Base` in `src/value.rs` | ONE | Hot path. Same `repr(u8)` layout. The merge adds only the schema impl. | `Base` in `src/value.rs` |
| `Sequence` in `src/variant/sequence.rs` | `Sequence` in `src/sequence.rs` | ONE | Same storage and text. | `Sequence` in `src/sequence.rs` |
| `Allele` in `src/variant/allele.rs` | `Allele` in `src/sequence.rs` | ONE | Same fields and checks. | `Allele` in `src/sequence.rs` |
| `SequenceLocation` in `src/variant/allele.rs` | `SequenceLocation` in `src/sequence.rs` | ONE-W | BioData's type keeps BioMCP's public fields and the check its serializer runs on `sequence`. Both write the same text. No schema additions. | `SequenceLocation` in `src/sequence.rs` |
| `GeneSymbol` in `src/id/gene.rs` | `GeneSymbol` in `src/value.rs` | ONE-W | Both accept the same alphabet and compare without case. BioData's type keeps BioMCP's 255-byte limit (`MAX_LEN`) and case-blind `Hash`, which narrows the second consumer's parse. No schema additions. | `GeneSymbol` in `src/gene_symbol.rs` |
| `Chromosome` in `src/coord/chromosome.rs` | `Chromosome` in `src/reference.rs` | ONE | Hot path. BioMCP copied this value; only the error type differs. | `Chromosome` in `src/reference.rs` |
| `Build` in `src/coord/genomic.rs` | `Build` in `src/reference.rs` | ONE | Same spellings `GRCh37` and `GRCh38`. | `Build` in `src/reference.rs` |
| `Strand` in `src/coord/genomic.rs` | `Strand` in `src/reference.rs` | ONE | Same `+` and `-` text; the second consumer adds `repr(i8)`. | `Strand` in `src/reference.rs` |
| `Contig` in `src/coord/genomic.rs` | `Contig` in `src/reference.rs` | ONE | The existing 255-byte bound also exposes the frozen `MAX_LEN` associated constant; only the error type differs. | `Contig` in `src/reference.rs` |
| `GenomicInterval` in `src/coord/genomic.rs` | `GenomicInterval` in `src/coordinate.rs` | ONE | Same fields; the second consumer derives `Copy`. | `GenomicInterval` in `src/coordinate.rs` |
| `Position1` in `src/coord.rs` | `Position1` in `src/coordinate.rs` | ONE | BioMCP stores `NonZeroU32`, the second consumer `u32` with a zero check. Wire and checks match; `Option<Position1>` grows from 4 to 8 bytes in BioMCP. | `Position1` in `src/coordinate.rs` |
| `Span0` in `src/coord.rs` | `Span0` in `src/coordinate.rs` | ONE | Same `start` and `end` object and order check. | `Span0` in `src/coordinate.rs` |
| `Bound` in `src/protein/coord.rs` | `Bound` in `src/uncertain.rs` | ONE | Same rule: at least one side, ordered. The second consumer splits the error into two cases. | `Bound` in `src/uncertain.rs` |
| `UncertainSpan0` in `src/protein/coord.rs` | `UncertainSpan0` in `src/uncertain.rs` | ONE | Same meaning and checks. Retain `from_inclusive`: refuse any supplied zero and reversed source limits before subtracting the start; use existing coordinate refusals. | `UncertainSpan0` in `src/uncertain.rs` |
| `Text` in `src/text.rs` | `Text` in `src/source_text.rs` | ONE-W | Keep BioMCP bounded control-safe parser, Hash and source escaping API with compact text storage. This narrows the second consumer parser. No schema addition. | `Text` in `src/source_text.rs` |
| `HgvsText` in `src/variant/notation.rs` | `HgvsText` in `src/source_text.rs` | ONE | Keep the frozen BioMCP body checks and protein-reference compatibility. The second parser narrows; HGVS parser layout and tags stay unchanged. | `HgvsText` in `src/source_text.rs` |
| `PartialDate` in `src/date.rs` | `PartialDate` in `src/source_text.rs` | ONE | Both check the calendar and write the same text. The merged value stores BioMCP's parts and preserves its `Copy`, `Hash`, `Display`, `new(u16, Option<u8>, Option<u8>) -> Result<Self, Error>`, `year(self) -> u16`, `month(self) -> Option<u8>` and `day(self) -> Option<u8>`. The second consumer's switch has no callers of the former borrowed-text accessor; callers use `Display`. Parsing retains the second consumer's `SourceTextError`. | `PartialDate` in `src/source_text.rs` |
| `FiniteNumber` in `src/number.rs` | `FiniteNumber` in `src/source_shapes.rs` | ONE | Same `f64` storage and wire. | `FiniteNumber` in `src/source_shapes.rs` |
| `Error` in `src/error.rs` | `Error` in `src/source_shapes.rs` | NAME | Distinct meanings. BioMCP's code enum keeps `Error`; the second consumer's two-case concept and article error becomes `SourceShapeError`. | `Error` in `src/error.rs`<br>`SourceShapeError` in `src/source_shapes.rs` |
| `Attribution` in `src/variant/statement/mod.rs` | `Attribution` in `src/attribution.rs` | ONE-W | The second consumer adds `approved` and `published` dates. See the schema additions table. | `Attribution` in `src/variant/statement/mod.rs` |
| `Concept` in `src/variant/statement/mod.rs` | `Concept` in `src/source_shapes.rs` | ONE | Same fields. | `Concept` in `src/variant/statement/mod.rs` |
| `Term` in `src/variant/statement/mod.rs` | `Term` in `src/source_shapes.rs` | ONE-W | Keep BioMCP label and codings list, omitted when empty. The second consumer maps its optional single coding to zero or one entries. No schema addition. | `Term` in `src/variant/statement/mod.rs` |
| `Curie` in `src/variant/statement/reference.rs` | `Curie` in `src/variant/statement/reference.rs` | ONE | PM ruling: adopt the frozen BioMCP grammar and schema. The second consumer refuses local parts containing `:` or beginning `/`. | `Curie` in `src/variant/statement/reference.rs` |
| `Direction` in `src/variant/statement/mod.rs` | `Direction` in `src/source_shapes.rs` | ONE | Same spellings. BioMCP marks it `non_exhaustive`; the merge keeps that. | `Direction` in `src/variant/statement/mod.rs` |
| `Predicate` in `src/variant/statement/mod.rs` | `Predicate` in `src/variant/statement/mod.rs` | ONE | Same declaration. | `Predicate` in `src/variant/statement/mod.rs` |
| `Proposition` in `src/variant/statement/mod.rs` | `Proposition` in `src/source_shapes.rs` | ONE-W | The second consumer adds four cases. See the schema additions table. | `Proposition` in `src/variant/statement/mod.rs` |
| `Statement` in `src/variant/statement/mod.rs` | `Statement` in `src/source_shapes.rs` | ONE-W | The second consumer adds four fields and gene and region subjects. See the schema additions table and settled subject ruling. | `Statement` in `src/variant/statement/mod.rs` |
| `VariantRef` in `src/variant/statement/reference.rs` | `VariantRef` in `src/variant_reference.rs` | ONE-W | The second consumer adds a `categorical` case. Its `protein.gene` stays `SourceGeneRef`, which holds every value of the second consumer's `GeneRef`. See the schema additions table. | `VariantRef` in `src/variant/statement/reference.rs` |
| `PredictorScore` in `src/variant/predictor.rs` | `PredictorScore` in `src/source_shapes.rs` | ONE-W | The second consumer adds `gene` and `transcript`. See the schema additions table. | `PredictorScore` in `src/variant/predictor.rs` |
| `Coding` in `src/variant/statement/coding.rs` | `Coding` in `src/coding/mod.rs` | ONE-W | Known conflict. Both write `{system, code}` with the same spellings for BioMCP's systems. BioData's type takes the union of systems; see the schema additions table. | `Coding` in `src/variant/statement/coding.rs` |
| `AcmgAmp2015Code` in `src/variant/statement/coding.rs` | `AcmgAmp2015Code` in `src/coding/vocab.rs` | ONE | Same code spellings. | `AcmgAmp2015Code` in `src/coding/vocab.rs` |
| `AmpAscoCap2017LevelCode` in `src/variant/statement/coding.rs` | `AmpAscoCap2017LevelCode` in `src/coding/vocab.rs` | ONE | Same code spellings. | `AmpAscoCap2017LevelCode` in `src/coding/vocab.rs` |
| `AmpAscoCap2017TierCode` in `src/variant/statement/coding.rs` | `AmpAscoCap2017TierCode` in `src/coding/vocab.rs` | ONE | Same code spellings. | `AmpAscoCap2017TierCode` in `src/coding/vocab.rs` |
| `CivicEvidenceLevelCode` in `src/variant/statement/coding.rs` | `CivicEvidenceLevelCode` in `src/coding/vocab.rs` | ONE | Same code spellings. | `CivicEvidenceLevelCode` in `src/coding/vocab.rs` |
| `ClingenCgcVicc2022Code` in `src/variant/statement/coding.rs` | `ClingenCgcVicc2022Code` in `src/coding/vocab.rs` | ONE | Same code spellings. | `ClingenCgcVicc2022Code` in `src/coding/vocab.rs` |
| `ClingenLowPenetrance2024Code` in `src/variant/statement/coding.rs` | `ClingenLowPenetrance2024Code` in `src/coding/vocab.rs` | ONE | Same code spellings. | `ClingenLowPenetrance2024Code` in `src/coding/vocab.rs` |
| `OncokbLevelCode` in `src/variant/statement/coding.rs` | `OncokbLevelCode` in `src/coding/vocab.rs` | ONE | Same code spellings. | `OncokbLevelCode` in `src/coding/vocab.rs` |
| `OncokbOncogenicityCode` in `src/variant/statement/coding.rs` | `OncokbOncogenicityCode` in `src/coding/vocab.rs` | ONE | Same code spellings. | `OncokbOncogenicityCode` in `src/coding/vocab.rs` |
| `PopulationFrequency` in `src/variant/population.rs` | `PopulationFrequency` in `src/population_frequency.rs` | ONE-W | Known conflict. Both have the same fields and refusals with the same codes. BioData's type keeps BioMCP's private fields and `Fraction` fractions; the second consumer's public fields and `f64` fractions adapt. No schema additions. | `PopulationFrequency` in `src/variant/population.rs` |
| `PopulationDataType` in `src/variant/population.rs` | `PopulationDataType` in `src/population_frequency.rs` | ONE | Same spellings. | `PopulationDataType` in `src/variant/population.rs` |
| `PopulationSex` in `src/variant/population.rs` | `PopulationSex` in `src/population_frequency.rs` | ONE | Same spellings. | `PopulationSex` in `src/variant/population.rs` |
| `GenotypeFilter` in `src/variant/population.rs` | `GenotypeFilter` in `src/population_frequency.rs` | ONE | Same single value `raw`. | `GenotypeFilter` in `src/variant/population.rs` |
| `Variant` in `src/variant/model.rs` | `Variant` in `src/variant/model.rs` | ONE-W | Both write published-text IDs. Preserve BioMCP genes as SourceGeneRef with source labels and optional versioned Ensembl IDs. The second consumer adapts its narrower gene references. No schema addition. | `Variant` in `src/variant/model.rs` |
| `Representation` in `src/variant/model.rs` | `Representation` in `src/variant/model.rs` | ONE | Same fields. | `Representation` in `src/variant/model.rs` |
| `Expression` in `src/variant/notation.rs` | `Expression` in `src/variant/notation.rs` | ONE | Same fields. Adopt frozen BioMCP body validation also for chromosome expressions. | `Expression` in `src/variant/notation.rs` |
| `ExpressionSyntax` in `src/variant/notation.rs` | `ExpressionSyntax` in `src/variant/notation.rs` | ONE | Same spellings. | `ExpressionSyntax` in `src/variant/notation.rs` |
| `RsId` in `src/variant/id.rs` | `RsId` in `src/identifier.rs` | ONE | Same grammar and text. Errors move to `IdError`, which gains `code()`. | `RsId` in `src/identifier.rs` |
| `PaId` in `src/variant/id.rs` | `PaId` in `src/identifier.rs` | ONE | Same grammar and text. Errors move to `IdError`, which gains `code()`. | `PaId` in `src/identifier.rs` |
| `ClinVarVariationId` in `src/variant/id.rs` | `ClinVarVariationId` in `src/identifier.rs` | ONE | Same grammar and text. BioMCP stores `NonZeroU64` and the second consumer `NonZeroU32`, so BioMCP's `new` and `get` change to `u32`. A property test against a current ClinVar release checks that range. Errors move to `IdError`, which gains `code()`. | `ClinVarVariationId` in `src/identifier.rs` |
| `ClinVarAlleleId` in `src/variant/id.rs` | `ClinVarAlleleId` in `src/identifier.rs` | ONE | Same grammar and text. BioMCP stores `NonZeroU64` and the second consumer `NonZeroU32`, so BioMCP's `new` and `get` change to `u32`. A property test against a current ClinVar release checks that range. Errors move to `IdError`, which gains `code()`. | `ClinVarAlleleId` in `src/identifier.rs` |
| `CaId` in `src/variant/id.rs` | `CaId` in `src/identifier.rs` | ONE | Both write at least six digits. | `CaId` in `src/identifier.rs` |
| `VcvId` in `src/variant/id.rs` | `VcvId` in `src/identifier.rs` | ONE | The second consumer stores number and optional version and writes the same text for nine-digit numbers. BioData's type refuses ten-digit numbers, as BioMCP does, which narrows the second consumer's parse. | `VcvId` in `src/identifier.rs` |
| `RcvId` in `src/variant/id.rs` | `RcvId` in `src/identifier.rs` | ONE | The second consumer stores number and optional version and writes the same text for nine-digit numbers. BioData's type refuses ten-digit numbers, as BioMCP does, which narrows the second consumer's parse. | `RcvId` in `src/identifier.rs` |
| `ScvId` in `src/variant/id.rs` | `ScvId` in `src/identifier.rs` | ONE | The second consumer stores number and optional version and writes the same text for nine-digit numbers. BioData's type refuses ten-digit numbers, as BioMCP does, which narrows the second consumer's parse. | `ScvId` in `src/identifier.rs` |
| `CosmicId` in `src/variant/id.rs` | `CosmicId` in `src/identifier.rs` | ONE | BioMCP keeps the text; the second consumer stores kind and number. The second consumer's `CosmicKind` becomes public to BioMCP. | `CosmicId` in `src/identifier.rs` |
| `VrsId` in `src/variant/id.rs` | `VrsId` in `src/identifier.rs` | ONE | Same storage. | `VrsId` in `src/identifier.rs` |
| `VariantId` in `src/variant/id.rs` | `VariantId` in `src/identifier.rs` | ONE | Same variants and text; the second consumer adds `Ord`. | `VariantId` in `src/identifier.rs` |
| `HgncId` in `src/id/mod.rs` | `HgncId` in `src/identifier.rs` | ONE | Same `HGNC:` text. | `HgncId` in `src/identifier.rs` |
| `NcbiGeneId` in `src/id/gene.rs` | `NcbiGeneId` in `src/identifier.rs` | ONE | Same text. | `NcbiGeneId` in `src/identifier.rs` |
| `OmimId` in `src/id/gene.rs` | `OmimId` in `src/identifier.rs` | ONE | BioData takes the second consumer's type. BioMCP's schema pattern stays as it is. | `OmimId` in `src/identifier.rs` |
| `Pmid` in `src/id/article.rs` | `Pmid` in `src/id/article.rs` | ONE | Same meaning; only the error type differs. | `Pmid` in `src/id/article.rs` |
| `GeneRef` in `src/gene/reference.rs` | `GeneRef` in `src/transcript.rs` | NAME | Known conflict. Two types stay. BioMCP's becomes `SourceGeneRef` and keeps schema name `GeneRef`; its label is any `Text` and its Ensembl gene ID keeps an optional version. The second consumer's keeps `GeneRef`, its `GeneSymbol` label and its unversioned Ensembl ID. | `GeneRef` in `src/transcript.rs`<br>`SourceGeneRef` in `src/gene/reference.rs` |
| `Mane` in `src/gene/annotations.rs` | `Mane` in `src/transcript.rs` | ONE | Same three parts. BioMCP has a public `status` field and serde. The merge makes `status` public and adds BioMCP's serde and schema impls behind their features. | `Mane` in `src/transcript.rs` |
| `ManeStatus` in `src/gene/annotations.rs` | `ManeStatus` in `src/transcript.rs` | ONE | Same values. | `ManeStatus` in `src/transcript.rs` |
| `UniProtFeature` in `src/protein/identity.rs` | `UniProtFeature` in `src/uniprot_feature.rs` | ONE | Same fields; lists change from `Box<[T]>` to `Vec<T>` in BioMCP code. | `UniProtFeature` in `src/uniprot_feature.rs` |
| `UniProtIsoform` in `src/protein/id.rs` | `UniProtIsoform` in `src/uniprot_feature.rs` | ONE | Same grammar and source-preserving `Display`. The merged constructor retains the second consumer's `new(impl Into<Box<str>>) -> Option<Self>` and borrowed `as_str`; BioMCP's frozen constructor returned `Result<Self, Error>`. | `UniProtIsoform` in `src/uniprot_feature.rs` |
| `AminoAcids` in `src/protein/id.rs` | `AminoAcids` in `src/uniprot_feature.rs` | ONE | Same grammar. | `AminoAcids` in `src/uniprot_feature.rs` |

| none | `NumberError` in `src/source_shapes.rs` | MOVE | BioMCP now returns its merged `Error::NumberNonFinite`; the second consumer keeps its typed refusal. | `NumberError` in `src/source_shapes.rs` |

### Items only in BioMCP's crate

| BioMCP | Second consumer | Rule | Note | Merged public item |
|---|---|---|---|---|
| `FaersDrug` in `src/adverse_event/faers.rs` | none | MOVE |  | `FaersDrug` in `src/adverse_event/faers.rs` |
| `FaersIdentifiers` in `src/adverse_event/faers.rs` | none | MOVE |  | `FaersIdentifiers` in `src/adverse_event/faers.rs` |
| `FaersPatient` in `src/adverse_event/faers.rs` | none | MOVE |  | `FaersPatient` in `src/adverse_event/faers.rs` |
| `FaersReaction` in `src/adverse_event/faers.rs` | none | MOVE |  | `FaersReaction` in `src/adverse_event/faers.rs` |
| `FaersReport` in `src/adverse_event/faers.rs` | none | MOVE |  | `FaersReport` in `src/adverse_event/faers.rs` |
| `FaersReporter` in `src/adverse_event/faers.rs` | none | MOVE |  | `FaersReporter` in `src/adverse_event/faers.rs` |
| `FaersSeriousness` in `src/adverse_event/faers.rs` | none | MOVE |  | `FaersSeriousness` in `src/adverse_event/faers.rs` |
| `MaudeDevice` in `src/adverse_event/maude.rs` | none | MOVE |  | `MaudeDevice` in `src/adverse_event/maude.rs` |
| `MaudeNarrative` in `src/adverse_event/maude.rs` | none | MOVE |  | `MaudeNarrative` in `src/adverse_event/maude.rs` |
| `MaudeReport` in `src/adverse_event/maude.rs` | none | MOVE |  | `MaudeReport` in `src/adverse_event/maude.rs` |
| `ReportId` in `src/adverse_event/mod.rs` | none | MOVE |  | `ReportId` in `src/adverse_event/mod.rs` |
| `AuthorId` in `src/author/id.rs` | none | MOVE |  | `AuthorId` in `src/author/id.rs` |
| `OrcidClientId` in `src/author/id.rs` | none | MOVE |  | `OrcidClientId` in `src/author/id.rs` |
| `OrcidId` in `src/author/id.rs` | none | MOVE |  | `OrcidId` in `src/author/id.rs` |
| `SemanticScholarAuthorId` in `src/author/id.rs` | none | MOVE |  | `SemanticScholarAuthorId` in `src/author/id.rs` |
| `OrcidAssertionSource` in `src/author/orcid.rs` | none | MOVE |  | `OrcidAssertionSource` in `src/author/orcid.rs` |
| `OrcidExternalId` in `src/author/orcid.rs` | none | MOVE |  | `OrcidExternalId` in `src/author/orcid.rs` |
| `OrcidPublicVisibility` in `src/author/orcid.rs` | none | MOVE |  | `OrcidPublicVisibility` in `src/author/orcid.rs` |
| `OrcidWorkAssertion` in `src/author/orcid.rs` | none | MOVE |  | `OrcidWorkAssertion` in `src/author/orcid.rs` |
| `OrcidWorkGroup` in `src/author/orcid.rs` | none | MOVE |  | `OrcidWorkGroup` in `src/author/orcid.rs` |
| `AuthorByline` in `src/author/records.rs` | none | MOVE |  | `AuthorByline` in `src/author/records.rs` |
| `AuthorCard` in `src/author/records.rs` | none | MOVE |  | `AuthorCard` in `src/author/records.rs` |
| `AuthorCountScope` in `src/author/records.rs` | none | MOVE |  | `AuthorCountScope` in `src/author/records.rs` |
| `AuthorCounts` in `src/author/records.rs` | none | MOVE |  | `AuthorCounts` in `src/author/records.rs` |
| `AuthorHeader` in `src/author/records.rs` | none | MOVE |  | `AuthorHeader` in `src/author/records.rs` |
| `AuthorPaperAccess` in `src/author/records.rs` | none | MOVE |  | `AuthorPaperAccess` in `src/author/records.rs` |
| `AuthorPaperDetail` in `src/author/records.rs` | none | MOVE |  | `AuthorPaperDetail` in `src/author/records.rs` |
| `AuthorPaperExternalId` in `src/author/records.rs` | none | MOVE |  | `AuthorPaperExternalId` in `src/author/records.rs` |
| `AuthorPaperHeader` in `src/author/records.rs` | none | MOVE |  | `AuthorPaperHeader` in `src/author/records.rs` |
| `CellLineSourceId` in `src/cell_line/id.rs` | none | MOVE |  | `CellLineSourceId` in `src/cell_line/id.rs` |
| `CellosaurusAccession` in `src/cell_line/id.rs` | none | MOVE |  | `CellosaurusAccession` in `src/cell_line/id.rs` |
| `ChemblCellId` in `src/cell_line/id.rs` | none | MOVE |  | `ChemblCellId` in `src/cell_line/id.rs` |
| `CellLineEntryDates` in `src/cell_line/records.rs` | none | MOVE |  | `CellLineEntryDates` in `src/cell_line/records.rs` |
| `CellLineGene` in `src/cell_line/records.rs` | none | MOVE |  | `CellLineGene` in `src/cell_line/records.rs` |
| `CellLineIdentity` in `src/cell_line/records.rs` | none | MOVE |  | `CellLineIdentity` in `src/cell_line/records.rs` |
| `CellLineName` in `src/cell_line/records.rs` | none | MOVE |  | `CellLineName` in `src/cell_line/records.rs` |
| `CellLineProvenance` in `src/cell_line/records.rs` | none | MOVE |  | `CellLineProvenance` in `src/cell_line/records.rs` |
| `CellLineReference` in `src/cell_line/records.rs` | none | MOVE |  | `CellLineReference` in `src/cell_line/records.rs` |
| `CellLineVariation` in `src/cell_line/records.rs` | none | MOVE |  | `CellLineVariation` in `src/cell_line/records.rs` |
| `CellLineWarning` in `src/cell_line/records.rs` | none | MOVE |  | `CellLineWarning` in `src/cell_line/records.rs` |
| `CellLineXref` in `src/cell_line/records.rs` | none | MOVE |  | `CellLineXref` in `src/cell_line/records.rs` |
| `CellLineSex` in `src/cell_line/sex.rs` | none | MOVE |  | `CellLineSex` in `src/cell_line/sex.rs` |
| `GtrConditionRelation` in `src/diagnostic/gtr.rs` | none | MOVE |  | `GtrConditionRelation` in `src/diagnostic/gtr.rs` |
| `GtrGeneRelation` in `src/diagnostic/gtr.rs` | none | MOVE |  | `GtrGeneRelation` in `src/diagnostic/gtr.rs` |
| `GtrMethod` in `src/diagnostic/gtr.rs` | none | MOVE |  | `GtrMethod` in `src/diagnostic/gtr.rs` |
| `GtrMethodList` in `src/diagnostic/gtr.rs` | none | MOVE |  | `GtrMethodList` in `src/diagnostic/gtr.rs` |
| `GtrTestRelation` in `src/diagnostic/gtr.rs` | none | MOVE |  | `GtrTestRelation` in `src/diagnostic/gtr.rs` |
| `GtrTestType` in `src/diagnostic/gtr.rs` | none | MOVE |  | `GtrTestType` in `src/diagnostic/gtr.rs` |
| `GtrTestVersion` in `src/diagnostic/gtr.rs` | none | MOVE |  | `GtrTestVersion` in `src/diagnostic/gtr.rs` |
| `DiagnosticDeviceId` in `src/diagnostic/id.rs` | none | MOVE |  | `DiagnosticDeviceId` in `src/diagnostic/id.rs` |
| `FdaDrugApplication` in `src/diagnostic/id.rs` | none | MOVE |  | `FdaDrugApplication` in `src/diagnostic/id.rs` |
| `GtrTestId` in `src/diagnostic/id.rs` | none | MOVE |  | `GtrTestId` in `src/diagnostic/id.rs` |
| `PmaNumber` in `src/diagnostic/id.rs` | none | MOVE |  | `PmaNumber` in `src/diagnostic/id.rs` |
| `PmaSupplement` in `src/diagnostic/id.rs` | none | MOVE |  | `PmaSupplement` in `src/diagnostic/id.rs` |
| `PmaSupplementCell` in `src/diagnostic/id.rs` | none | MOVE |  | `PmaSupplementCell` in `src/diagnostic/id.rs` |
| `WhoProductId` in `src/diagnostic/id.rs` | none | MOVE |  | `WhoProductId` in `src/diagnostic/id.rs` |
| `DeviceRecallNumber` in `src/diagnostic/id.rs` | none | MOVE | | `DeviceRecallNumber` in `src/diagnostic/id.rs` |
| `DiagnosticRecord` in `src/diagnostic/mod.rs` | none | MOVE |  | `DiagnosticRecord` in `src/diagnostic/mod.rs` |
| `CompanionDiagnosticRelation` in `src/diagnostic/regulatory.rs` | none | MOVE |  | `CompanionDiagnosticRelation` in `src/diagnostic/regulatory.rs` |
| `CompanionGroupCells` in `src/diagnostic/regulatory.rs` | none | MOVE |  | `CompanionGroupCells` in `src/diagnostic/regulatory.rs` |
| `CompanionIndividualCells` in `src/diagnostic/regulatory.rs` | none | MOVE |  | `CompanionIndividualCells` in `src/diagnostic/regulatory.rs` |
| `DeviceSubmission` in `src/diagnostic/regulatory.rs` | none | MOVE |  | `DeviceSubmission` in `src/diagnostic/regulatory.rs` |
| `DiagnosticCell` in `src/diagnostic/regulatory.rs` | none | MOVE |  | `DiagnosticCell` in `src/diagnostic/regulatory.rs` |
| `DiagnosticCellLink` in `src/diagnostic/regulatory.rs` | none | MOVE |  | `DiagnosticCellLink` in `src/diagnostic/regulatory.rs` |
| `DiagnosticDrugApplication` in `src/diagnostic/regulatory.rs` | none | MOVE |  | `DiagnosticDrugApplication` in `src/diagnostic/regulatory.rs` |
| `PmaIdentity` in `src/diagnostic/regulatory.rs` | none | MOVE |  | `PmaIdentity` in `src/diagnostic/regulatory.rs` |
| `WhoIvdProduct` in `src/diagnostic/regulatory.rs` | none | MOVE |  | `WhoIvdProduct` in `src/diagnostic/regulatory.rs` |
| `WhoIvdRow` in `src/diagnostic/regulatory.rs` | none | MOVE |  | `WhoIvdRow` in `src/diagnostic/regulatory.rs` |
| `DiseaseAssociation` in `src/disease.rs` | none | MOVE |  | `DiseaseAssociation` in `src/disease.rs` |
| `DiseaseAssociationQualifier` in `src/disease.rs` | none | MOVE |  | `DiseaseAssociationQualifier` in `src/disease.rs` |
| `DiseaseAssociationReference` in `src/disease.rs` | none | MOVE |  | `DiseaseAssociationReference` in `src/disease.rs` |
| `DiseaseEndpoint` in `src/disease.rs` | none | MOVE |  | `DiseaseEndpoint` in `src/disease.rs` |
| `DiseaseRelation` in `src/disease.rs` | none | MOVE |  | `DiseaseRelation` in `src/disease.rs` |
| `DiseaseStatistic` in `src/disease.rs` | none | MOVE |  | `DiseaseStatistic` in `src/disease.rs` |
| `DiseaseSynonym` in `src/disease.rs` | none | MOVE |  | `DiseaseSynonym` in `src/disease.rs` |
| `StatisticEstimate` in `src/disease.rs` | none | MOVE |  | `StatisticEstimate` in `src/disease.rs` |
| `StatisticSelector` in `src/disease.rs` | none | MOVE |  | `StatisticSelector` in `src/disease.rs` |
| `DrugIndicationClaim` in `src/drug/claims.rs` | none | MOVE |  | `DrugIndicationClaim` in `src/drug/claims.rs` |
| `DrugLabelReference` in `src/drug/claims.rs` | none | MOVE |  | `DrugLabelReference` in `src/drug/claims.rs` |
| `DrugLabelSection` in `src/drug/claims.rs` | none | MOVE |  | `DrugLabelSection` in `src/drug/claims.rs` |
| `DrugId` in `src/drug/id.rs` | none | MOVE |  | `DrugId` in `src/drug/id.rs` |
| `DrugIdentity` in `src/drug/identity.rs` | none | MOVE |  | `DrugIdentity` in `src/drug/identity.rs` |
| `DrugIdentityLevel` in `src/drug/identity.rs` | none | MOVE |  | `DrugIdentityLevel` in `src/drug/identity.rs` |
| `DrugIdentityRelation` in `src/drug/identity.rs` | none | MOVE |  | `DrugIdentityRelation` in `src/drug/identity.rs` |
| `DrugProduct` in `src/drug/identity.rs` | none | MOVE |  | `DrugProduct` in `src/drug/identity.rs` |
| `DrugInteractionClaim` in `src/drug/interaction.rs` | none | MOVE |  | `DrugInteractionClaim` in `src/drug/interaction.rs` |
| `DrugDiseaseReference` in `src/drug/native.rs` | none | MOVE |  | `DrugDiseaseReference` in `src/drug/native.rs` |
| `DrugNativeReference` in `src/drug/native.rs` | none | MOVE |  | `DrugNativeReference` in `src/drug/native.rs` |
| `DrugNativeTarget` in `src/drug/native.rs` | none | MOVE |  | `DrugNativeTarget` in `src/drug/native.rs` |
| `DrugStage` in `src/drug/native.rs` | none | MOVE |  | `DrugStage` in `src/drug/native.rs` |
| `AuthorizationDate` in `src/drug/regulatory.rs` | none | MOVE |  | `AuthorizationDate` in `src/drug/regulatory.rs` |
| `AuthorizationEvent` in `src/drug/regulatory.rs` | none | MOVE |  | `AuthorizationEvent` in `src/drug/regulatory.rs` |
| `DrugTargetRelation` in `src/drug/target.rs` | none | MOVE |  | `DrugTargetRelation` in `src/drug/target.rs` |
| `Summary` in `src/gene/annotations.rs` | none | MOVE |  | `Summary` in `src/gene/annotations.rs` |
| `GeneLocation` in `src/gene/location.rs` | none | MOVE | | `GeneLocation` in `src/gene/location.rs` |
| `GeneLocationKind` in `src/gene/location.rs` | none | MOVE | | `GeneLocationKind` in `src/gene/location.rs` |
| `Gene` in `src/gene/model.rs` | none | MOVE |  | `Gene` in `src/gene/model.rs` |
| `GeneStatus` in `src/gene/values.rs` | none | MOVE |  | `GeneStatus` in `src/gene/values.rs` |
| `HgncStatus` in `src/gene/values.rs` | none | MOVE |  | `HgncStatus` in `src/gene/values.rs` |
| `LocusGroup` in `src/gene/values.rs` | none | MOVE |  | `LocusGroup` in `src/gene/values.rs` |
| `LocusType` in `src/gene/values.rs` | none | MOVE |  | `LocusType` in `src/gene/values.rs` |
| `MatchKind` in `src/gene/values.rs` | none | MOVE |  | `MatchKind` in `src/gene/values.rs` |
| `SummarySource` in `src/gene/values.rs` | none | MOVE |  | `SummarySource` in `src/gene/values.rs` |
| `GwasAssociationId` in `src/gwas.rs` | none | MOVE |  | `GwasAssociationId` in `src/gwas.rs` |
| `GwasStudyId` in `src/gwas.rs` | none | MOVE |  | `GwasStudyId` in `src/gwas.rs` |
| `GwasTraitId` in `src/gwas.rs` | none | MOVE |  | `GwasTraitId` in `src/gwas.rs` |
| `ScientificPValue` in `src/gwas/probability.rs` | none | MOVE |  | `ScientificPValue` in `src/gwas/probability.rs` |
| `Doi` in `src/id/article.rs` | none | MOVE |  | `Doi` in `src/id/article.rs` |
| `EuropePmcId` in `src/id/article.rs` | none | MOVE |  | `EuropePmcId` in `src/id/article.rs` |
| `Pmcid` in `src/id/article.rs` | none | MOVE |  | `Pmcid` in `src/id/article.rs` |
| `SemanticScholarId` in `src/id/article.rs` | none | MOVE |  | `SemanticScholarId` in `src/id/article.rs` |
| `Cytoband` in `src/id/gene.rs` | none | MOVE | Distinct from ISCN `Band`: one band identifier, not a karyotype part. | `Cytoband` in `src/id/gene.rs` |
| `UniProtAcc` in `src/id/gene.rs` | none | MOVE |  | `UniProtAcc` in `src/id/gene.rs` |
| `EnsemblGeneId` in `src/id/transcript.rs` | none | MOVE | Stays, as the Ensembl gene ID of `SourceGeneRef`. | `EnsemblGeneId` in `src/id/transcript.rs` |
| `absent_or_empty` in `src/lib.rs` | none | MOVE |  | `absent_or_empty` in `src/lib.rs` |
| `License` in `src/license.rs` | none | MOVE |  | `License` in `src/license.rs` |
| `canonical_enum` in `src/macros.rs` | none | MOVE | Exported macro that BioMCP's other crates call. | `canonical_enum` in `src/macros.rs` |
| `native_enum` in `src/macros.rs` | none | MOVE | Exported macro that BioMCP's other crates call. | `native_enum` in `src/macros.rs` |
| `text_value` in `src/macros.rs` | none | MOVE | Exported macro that BioMCP's other crates call. | `text_value` in `src/macros.rs` |
| `wire_enum` in `src/macros.rs` | none | MOVE | Exported macro that BioMCP's other crates call. | `wire_enum` in `src/macros.rs` |
| `Markdown` in `src/markdown.rs` | none | MOVE |  | `Markdown` in `src/markdown.rs` |
| `MondoId` in `src/ontology_id.rs` | none | MOVE |  | `MondoId` in `src/ontology_id.rs` |
| `NcitId` in `src/ontology_id.rs` | none | MOVE |  | `NcitId` in `src/ontology_id.rs` |
| `PathwayId` in `src/pathway/ids.rs` | none | MOVE |  | `PathwayId` in `src/pathway/ids.rs` |
| `PathwayRef` in `src/pathway/ids.rs` | none | MOVE |  | `PathwayRef` in `src/pathway/ids.rs` |
| `ReactomeId` in `src/pathway/ids.rs` | none | MOVE |  | `ReactomeId` in `src/pathway/ids.rs` |
| `WikiPathwaysId` in `src/pathway/ids.rs` | none | MOVE |  | `WikiPathwaysId` in `src/pathway/ids.rs` |
| `Pathway` in `src/pathway/mod.rs` | none | MOVE |  | `Pathway` in `src/pathway/mod.rs` |
| `PathwayEventRef` in `src/pathway/mod.rs` | none | MOVE |  | `PathwayEventRef` in `src/pathway/mod.rs` |
| `PathwayLiteratureRef` in `src/pathway/mod.rs` | none | MOVE |  | `PathwayLiteratureRef` in `src/pathway/mod.rs` |
| `PathwayMember` in `src/pathway/mod.rs` | none | MOVE |  | `PathwayMember` in `src/pathway/mod.rs` |
| `PathwayParticipant` in `src/pathway/mod.rs` | none | MOVE |  | `PathwayParticipant` in `src/pathway/mod.rs` |
| `PathwaySourceRef` in `src/pathway/mod.rs` | none | MOVE |  | `PathwaySourceRef` in `src/pathway/mod.rs` |
| `PathwaySpecies` in `src/pathway/mod.rs` | none | MOVE |  | `PathwaySpecies` in `src/pathway/mod.rs` |
| `PathwaySummation` in `src/pathway/mod.rs` | none | MOVE |  | `PathwaySummation` in `src/pathway/mod.rs` |
| `ClinPgxAccession` in `src/pgx/accession.rs` | none | MOVE |  | `ClinPgxAccession` in `src/pgx/accession.rs` |
| `PgxDiplotypeExpression` in `src/pgx/accession.rs` | none | MOVE |  | `PgxDiplotypeExpression` in `src/pgx/accession.rs` |
| `PgxGeneKey` in `src/pgx/accession.rs` | none | MOVE |  | `PgxGeneKey` in `src/pgx/accession.rs` |
| `PgxAnnotationHeader` in `src/pgx/annotation.rs` | none | MOVE |  | `PgxAnnotationHeader` in `src/pgx/annotation.rs` |
| `PgxAnnotationReference` in `src/pgx/annotation.rs` | none | MOVE |  | `PgxAnnotationReference` in `src/pgx/annotation.rs` |
| `PgxCrossReference` in `src/pgx/annotation.rs` | none | MOVE |  | `PgxCrossReference` in `src/pgx/annotation.rs` |
| `PgxDiplotypeReference` in `src/pgx/annotation.rs` | none | MOVE |  | `PgxDiplotypeReference` in `src/pgx/annotation.rs` |
| `PgxEvidenceTerm` in `src/pgx/annotation.rs` | none | MOVE |  | `PgxEvidenceTerm` in `src/pgx/annotation.rs` |
| `PgxLocation` in `src/pgx/annotation.rs` | none | MOVE |  | `PgxLocation` in `src/pgx/annotation.rs` |
| `PgxObjectReference` in `src/pgx/annotation.rs` | none | MOVE |  | `PgxObjectReference` in `src/pgx/annotation.rs` |
| `PgxUnobserved` in `src/pgx/annotation.rs` | none | MOVE |  | `PgxUnobserved` in `src/pgx/annotation.rs` |
| `PgxDiplotypeResult` in `src/pgx/cpic.rs` | none | MOVE |  | `PgxDiplotypeResult` in `src/pgx/cpic.rs` |
| `PgxDrugRecommendation` in `src/pgx/cpic.rs` | none | MOVE |  | `PgxDrugRecommendation` in `src/pgx/cpic.rs` |
| `PgxGeneIdentity` in `src/pgx/cpic.rs` | none | MOVE |  | `PgxGeneIdentity` in `src/pgx/cpic.rs` |
| `PgxGuidelineGene` in `src/pgx/cpic.rs` | none | MOVE |  | `PgxGuidelineGene` in `src/pgx/cpic.rs` |
| `PgxGuidelineReference` in `src/pgx/cpic.rs` | none | MOVE |  | `PgxGuidelineReference` in `src/pgx/cpic.rs` |
| `PgxPair` in `src/pgx/cpic.rs` | none | MOVE |  | `PgxPair` in `src/pgx/cpic.rs` |
| `PgxRecommendation` in `src/pgx/cpic.rs` | none | MOVE |  | `PgxRecommendation` in `src/pgx/cpic.rs` |
| `PgxAllelePhenotype` in `src/pgx/detail.rs` | none | MOVE |  | `PgxAllelePhenotype` in `src/pgx/detail.rs` |
| `PgxAnnotationDetail` in `src/pgx/detail.rs` | none | MOVE |  | `PgxAnnotationDetail` in `src/pgx/detail.rs` |
| `PgxAnnotationHistory` in `src/pgx/detail.rs` | none | MOVE |  | `PgxAnnotationHistory` in `src/pgx/detail.rs` |
| `HpId` in `src/phenotype.rs` | none | MOVE |  | `HpId` in `src/phenotype.rs` |
| `PhenotypeAssociationCount` in `src/phenotype.rs` | none | MOVE |  | `PhenotypeAssociationCount` in `src/phenotype.rs` |
| `PhenotypeCard` in `src/phenotype.rs` | none | MOVE |  | `PhenotypeCard` in `src/phenotype.rs` |
| `PhenotypeHeader` in `src/phenotype.rs` | none | MOVE |  | `PhenotypeHeader` in `src/phenotype.rs` |
| `PhenotypeHierarchyLink` in `src/phenotype.rs` | none | MOVE |  | `PhenotypeHierarchyLink` in `src/phenotype.rs` |
| `PhenotypeHierarchyRelation` in `src/phenotype.rs` | none | MOVE |  | `PhenotypeHierarchyRelation` in `src/phenotype.rs` |
| `PhenotypeSynonym` in `src/phenotype.rs` | none | MOVE |  | `PhenotypeSynonym` in `src/phenotype.rs` |
| `PhenotypeSynonymClass` in `src/phenotype.rs` | none | MOVE |  | `PhenotypeSynonymClass` in `src/phenotype.rs` |
| `PhenotypeXref` in `src/phenotype.rs` | none | MOVE |  | `PhenotypeXref` in `src/phenotype.rs` |
| `ProteinComplexHeader` in `src/protein/complex.rs` | none | MOVE |  | `ProteinComplexHeader` in `src/protein/complex.rs` |
| `ProteinComplexMember` in `src/protein/complex.rs` | none | MOVE |  | `ProteinComplexMember` in `src/protein/complex.rs` |
| `ProteinDomainFragment` in `src/protein/domain.rs` | none | MOVE |  | `ProteinDomainFragment` in `src/protein/domain.rs` |
| `ProteinDomainLocation` in `src/protein/domain.rs` | none | MOVE |  | `ProteinDomainLocation` in `src/protein/domain.rs` |
| `ProteinDomainMatch` in `src/protein/domain.rs` | none | MOVE |  | `ProteinDomainMatch` in `src/protein/domain.rs` |
| `ProteinMemberSignature` in `src/protein/domain.rs` | none | MOVE |  | `ProteinMemberSignature` in `src/protein/domain.rs` |
| `ComplexPortalId` in `src/protein/id.rs` | none | MOVE |  | `ComplexPortalId` in `src/protein/id.rs` |
| `InterProId` in `src/protein/id.rs` | none | MOVE |  | `InterProId` in `src/protein/id.rs` |
| `Protein` in `src/protein/identity.rs` | none | MOVE |  | `Protein` in `src/protein/identity.rs` |
| `ProteinFeature` in `src/protein/identity.rs` | none | MOVE |  | `ProteinFeature` in `src/protein/identity.rs` |
| `ProteinHeader` in `src/protein/identity.rs` | none | MOVE |  | `ProteinHeader` in `src/protein/identity.rs` |
| `ProteinIsoform` in `src/protein/identity.rs` | none | MOVE |  | `ProteinIsoform` in `src/protein/identity.rs` |
| `ProteinCoordinateConvention` in `src/protein/mod.rs` | none | MOVE |  | `ProteinCoordinateConvention` in `src/protein/mod.rs` |
| `ProteinIsoformStatus` in `src/protein/mod.rs` | none | MOVE |  | `ProteinIsoformStatus` in `src/protein/mod.rs` |
| `ProteinSequenceAvailability` in `src/protein/mod.rs` | none | MOVE |  | `ProteinSequenceAvailability` in `src/protein/mod.rs` |
| `ProteinAudit` in `src/protein/sequence.rs` | none | MOVE |  | `ProteinAudit` in `src/protein/sequence.rs` |
| `ProteinFunction` in `src/protein/sequence.rs` | none | MOVE |  | `ProteinFunction` in `src/protein/sequence.rs` |
| `ProteinSequence` in `src/protein/sequence.rs` | none | MOVE |  | `ProteinSequence` in `src/protein/sequence.rs` |
| `ProteinStructureProperty` in `src/protein/sequence.rs` | none | MOVE |  | `ProteinStructureProperty` in `src/protein/sequence.rs` |
| `ProteinStructureRef` in `src/protein/sequence.rs` | none | MOVE |  | `ProteinStructureRef` in `src/protein/sequence.rs` |
| `SourceText` in `src/source_text.rs` | none | MOVE | Distinct from `Text`: keeps whitespace and empty values. | `SourceText` in `src/source_text_value.rs` |
| `CbioportalCancerTypeId` in `src/study/id.rs` | none | MOVE |  | `CbioportalCancerTypeId` in `src/study/id.rs` |
| `StudyBlobSha` in `src/study/id.rs` | none | MOVE |  | `StudyBlobSha` in `src/study/id.rs` |
| `StudyId` in `src/study/id.rs` | none | MOVE |  | `StudyId` in `src/study/id.rs` |
| `StudyImportTime` in `src/study/id.rs` | none | MOVE |  | `StudyImportTime` in `src/study/id.rs` |
| `StudyProfileId` in `src/study/id.rs` | none | MOVE |  | `StudyProfileId` in `src/study/id.rs` |
| `StudyResourceId` in `src/study/id.rs` | none | MOVE |  | `StudyResourceId` in `src/study/id.rs` |
| `StudySampleListId` in `src/study/id.rs` | none | MOVE |  | `StudySampleListId` in `src/study/id.rs` |
| `Study` in `src/study/mod.rs` | none | MOVE |  | `Study` in `src/study/mod.rs` |
| `StudyHeader` in `src/study/mod.rs` | none | MOVE |  | `StudyHeader` in `src/study/mod.rs` |
| `StudyResourceCount` in `src/study/mod.rs` | none | MOVE |  | `StudyResourceCount` in `src/study/mod.rs` |
| `StudyFile` in `src/study/sections.rs` | none | MOVE |  | `StudyFile` in `src/study/sections.rs` |
| `StudyFileKind` in `src/study/sections.rs` | none | MOVE |  | `StudyFileKind` in `src/study/sections.rs` |
| `StudyProfile` in `src/study/sections.rs` | none | MOVE |  | `StudyProfile` in `src/study/sections.rs` |
| `StudySampleList` in `src/study/sections.rs` | none | MOVE |  | `StudySampleList` in `src/study/sections.rs` |
| `NctId` in `src/trial.rs` | none | MOVE |  | `NctId` in `src/trial.rs` |
| `StudyType` in `src/trial.rs` | none | MOVE |  | `StudyType` in `src/trial.rs` |
| `TrialPhase` in `src/trial.rs` | none | MOVE |  | `TrialPhase` in `src/trial.rs` |
| `TrialSex` in `src/trial.rs` | none | MOVE |  | `TrialSex` in `src/trial.rs` |
| `TrialStatus` in `src/trial.rs` | none | MOVE |  | `TrialStatus` in `src/trial.rs` |
| `TrialAgeBound` in `src/trial_detail.rs` | none | MOVE |  | `TrialAgeBound` in `src/trial_detail.rs` |
| `TrialArm` in `src/trial_detail.rs` | none | MOVE |  | `TrialArm` in `src/trial_detail.rs` |
| `TrialArmKind` in `src/trial_detail.rs` | none | MOVE |  | `TrialArmKind` in `src/trial_detail.rs` |
| `TrialCard` in `src/trial_detail.rs` | none | MOVE |  | `TrialCard` in `src/trial_detail.rs` |
| `TrialDate` in `src/trial_detail.rs` | none | MOVE |  | `TrialDate` in `src/trial_detail.rs` |
| `TrialDateKind` in `src/trial_detail.rs` | none | MOVE |  | `TrialDateKind` in `src/trial_detail.rs` |
| `TrialEligibility` in `src/trial_detail.rs` | none | MOVE |  | `TrialEligibility` in `src/trial_detail.rs` |
| `TrialIntervention` in `src/trial_detail.rs` | none | MOVE |  | `TrialIntervention` in `src/trial_detail.rs` |
| `TrialInterventionKind` in `src/trial_detail.rs` | none | MOVE |  | `TrialInterventionKind` in `src/trial_detail.rs` |
| `TrialOutcome` in `src/trial_detail.rs` | none | MOVE |  | `TrialOutcome` in `src/trial_detail.rs` |
| `TrialOutcomeKind` in `src/trial_detail.rs` | none | MOVE |  | `TrialOutcomeKind` in `src/trial_detail.rs` |
| `TrialDocument` in `src/trial_publications.rs` | none | MOVE |  | `TrialDocument` in `src/trial_publications.rs` |
| `TrialReference` in `src/trial_publications.rs` | none | MOVE |  | `TrialReference` in `src/trial_publications.rs` |
| `TrialContact` in `src/trial_sites.rs` | none | MOVE |  | `TrialContact` in `src/trial_sites.rs` |
| `TrialContactRole` in `src/trial_sites.rs` | none | MOVE |  | `TrialContactRole` in `src/trial_sites.rs` |
| `TrialCoordinates` in `src/trial_sites.rs` | none | MOVE |  | `TrialCoordinates` in `src/trial_sites.rs` |
| `TrialLocation` in `src/trial_sites.rs` | none | MOVE |  | `TrialLocation` in `src/trial_sites.rs` |
| `TrialOfficial` in `src/trial_sites.rs` | none | MOVE |  | `TrialOfficial` in `src/trial_sites.rs` |
| `TrialOfficialRole` in `src/trial_sites.rs` | none | MOVE |  | `TrialOfficialRole` in `src/trial_sites.rs` |
| `TrialSiteStatus` in `src/trial_sites.rs` | none | MOVE |  | `TrialSiteStatus` in `src/trial_sites.rs` |
| `Fraction` in `src/number.rs` | none | MOVE | A finite number from zero to one. BioData's `PopulationFrequency` fractions use it. | `Fraction` in `src/number.rs` |

| `ClinvarAlleleOriginCode` in `src/variant/statement/clinvar.rs` | `AlleleOrigin` in `src/clinvar_value.rs` | ONE | Keep publisher-label FromStr, named from_code constructor, and schema name `ClinvarAlleleOriginCode`; BioMCP adapts construction. Existing case bytes remain identical. | `AlleleOrigin` in `src/clinvar_value.rs` |
| `ClinvarReviewStatusCode` in `src/variant/statement/clinvar.rs` | `ReviewStatus` in `src/clinvar_value.rs` | ONE | Keep publisher-label FromStr, named from_code constructor, and schema name `ClinvarReviewStatusCode`; BioMCP adapts construction. Existing case bytes remain identical. | `ReviewStatus` in `src/clinvar_value.rs` |
| `ClinvarHaplotype` in `src/variant/statement/haplotype.rs` | none | MOVE |  | `ClinvarHaplotype` in `src/variant/statement/haplotype.rs` |
| `ClinvarHaplotypeMember` in `src/variant/statement/haplotype.rs` | none | MOVE |  | `ClinvarHaplotypeMember` in `src/variant/statement/haplotype.rs` |
| `ProfileExpression` in `src/variant/statement/profile.rs` | none | MOVE |  | `ProfileExpression` in `src/variant/statement/profile.rs` |
| `ProfileNode` in `src/variant/statement/profile.rs` | none | MOVE |  | `ProfileNode` in `src/variant/statement/profile.rs` |
| `CodingChange` in `src/variant/coding_change.rs` | none | MOVE |  | `CodingChange` in `src/variant/coding_change.rs` |
| `DrugRecallNumber` in `src/diagnostic/id.rs` | none | MOVE |  | `DrugRecallNumber` in `src/diagnostic/id.rs` |
| `FaersAgeUnit` in `src/adverse_event/age.rs` | none | MOVE |  | `FaersAgeUnit` in `src/adverse_event/age.rs` |
| `GtrConditionReference` in `src/diagnostic/gtr.rs` | none | MOVE |  | `GtrConditionReference` in `src/diagnostic/gtr.rs` |
| `GtrGeneReference` in `src/diagnostic/gtr.rs` | none | MOVE |  | `GtrGeneReference` in `src/diagnostic/gtr.rs` |
| `Identifier` in `src/identifier.rs` | none | MOVE |  | `Identifier` in `src/source_identifier.rs` |
| `IdentifierNamespace` in `src/identifier.rs` | none | MOVE |  | `IdentifierNamespace` in `src/source_identifier.rs` |
| `TrialReferenceKind` in `src/trial_publications.rs` | none | MOVE |  | `TrialReferenceKind` in `src/trial_publications.rs` |
| `TrialUploadTime` in `src/trial_upload_time.rs` | none | MOVE |  | `TrialUploadTime` in `src/trial_upload_time.rs` |
| `WHITESPACE` in `src/schema_pattern.rs` | none | MOVE |  | `WHITESPACE` in `src/schema_pattern.rs` |
| `decimal` in `src/schema_pattern.rs` | none | MOVE |  | `decimal` in `src/schema_pattern.rs` |
| `padded_decimal` in `src/schema_pattern.rs` | none | MOVE |  | `padded_decimal` in `src/schema_pattern.rs` |
| `source_decimal` in `src/schema_pattern.rs` | none | MOVE |  | `source_decimal` in `src/schema_pattern.rs` |
| `schema_pattern` in `src/schema_pattern.rs` | none | MOVE |  | `schema_pattern` in `src/schema_pattern.rs` |
| `WithUnit` in `src/number.rs` | none | MOVE |  | `WithUnit` in `src/number.rs` |
| `checked_id` in `src/macros.rs` | none | MOVE |  | `checked_id` in `src/macros.rs` |

### Items only in the second consumer's crate

| BioMCP | Second consumer | Rule | Note | Merged public item |
|---|---|---|---|---|
| none | `CopyChange` in `src/alteration/cytogenetic.rs` | MOVE |  | `CopyChange` in `src/alteration/cytogenetic.rs` |
| none | `CountKind` in `src/alteration/cytogenetic.rs` | MOVE |  | `CountKind` in `src/alteration/cytogenetic.rs` |
| none | `Cytogenetic` in `src/alteration/cytogenetic.rs` | MOVE |  | `Cytogenetic` in `src/alteration/cytogenetic.rs` |
| none | `KaryotypeClass` in `src/alteration/cytogenetic.rs` | MOVE |  | `KaryotypeClass` in `src/alteration/cytogenetic.rs` |
| none | `Ploidy` in `src/alteration/cytogenetic.rs` | MOVE |  | `Ploidy` in `src/alteration/cytogenetic.rs` |
| none | `Somy` in `src/alteration/cytogenetic.rs` | MOVE |  | `Somy` in `src/alteration/cytogenetic.rs` |
| none | `Category` in `src/alteration/event.rs` | MOVE |  | `Category` in `src/alteration/event.rs` |
| none | `Effect` in `src/alteration/event.rs` | MOVE |  | `Effect` in `src/alteration/event.rs` |
| none | `GeneEvent` in `src/alteration/event.rs` | MOVE |  | `GeneEvent` in `src/alteration/event.rs` |
| none | `Scope` in `src/alteration/event.rs` | MOVE |  | `Scope` in `src/alteration/event.rs` |
| none | `Comparison` in `src/alteration/marker.rs` | MOVE |  | `Comparison` in `src/alteration/marker.rs` |
| none | `Decimal` in `src/alteration/marker.rs` | MOVE |  | `Decimal` in `src/alteration/marker.rs` |
| none | `ExpressionState` in `src/alteration/marker.rs` | MOVE |  | `ExpressionState` in `src/alteration/marker.rs` |
| none | `Finding` in `src/alteration/marker.rs` | MOVE |  | `Finding` in `src/alteration/marker.rs` |
| none | `Level` in `src/alteration/marker.rs` | MOVE |  | `Level` in `src/alteration/marker.rs` |
| none | `Mmr` in `src/alteration/marker.rs` | MOVE |  | `Mmr` in `src/alteration/marker.rs` |
| none | `Msi` in `src/alteration/marker.rs` | MOVE |  | `Msi` in `src/alteration/marker.rs` |
| none | `Score` in `src/alteration/marker.rs` | MOVE |  | `Score` in `src/alteration/marker.rs` |
| none | `ScoreBound` in `src/alteration/marker.rs` | MOVE |  | `ScoreBound` in `src/alteration/marker.rs` |
| none | `ScoringSystem` in `src/alteration/marker.rs` | MOVE |  | `ScoringSystem` in `src/alteration/marker.rs` |
| none | `Signature` in `src/alteration/marker.rs` | MOVE |  | `Signature` in `src/alteration/marker.rs` |
| none | `Alteration` in `src/alteration/mod.rs` | MOVE |  | `Alteration` in `src/alteration/mod.rs` |
| none | `AlterationCode` in `src/alteration/mod.rs` | MOVE |  | `AlterationCode` in `src/alteration/mod.rs` |
| none | `Change` in `src/alteration/mod.rs` | MOVE | Distinct from `hgvs::Change`. | `Change` in `src/alteration/mod.rs` |
| none | `Kind` in `src/alteration/mod.rs` | MOVE |  | `Kind` in `src/alteration/mod.rs` |
| none | `Origin` in `src/alteration/mod.rs` | MOVE |  | `Origin` in `src/alteration/mod.rs` |
| none | `alteration` in `src/alteration/mod.rs` | MOVE |  | `alteration` in `src/alteration/mod.rs` |
| none | `parse` in `src/alteration/mod.rs` | MOVE |  | `parse` in `src/alteration/mod.rs` |
| none | `HlaSuffix` in `src/alteration/pharmacogenomic.rs` | MOVE |  | `HlaSuffix` in `src/alteration/pharmacogenomic.rs` |
| none | `Star` in `src/alteration/pharmacogenomic.rs` | MOVE |  | `Star` in `src/alteration/pharmacogenomic.rs` |
| none | `StarSuffix` in `src/alteration/pharmacogenomic.rs` | MOVE |  | `StarSuffix` in `src/alteration/pharmacogenomic.rs` |
| none | `AlterationReferenceError` in `src/alteration_reference.rs` | MOVE |  | `AlterationReferenceError` in `src/alteration_reference.rs` |
| none | `Consequence` in `src/annotation.rs` | MOVE |  | `Consequence` in `src/annotation.rs` |
| none | `ConsequenceError` in `src/annotation.rs` | MOVE |  | `ConsequenceError` in `src/annotation.rs` |
| none | `GeneExtent` in `src/annotation.rs` | MOVE |  | `GeneExtent` in `src/annotation.rs` |
| none | `NmdEscape` in `src/annotation.rs` | MOVE |  | `NmdEscape` in `src/annotation.rs` |
| none | `NmdEscapeReason` in `src/annotation.rs` | MOVE |  | `NmdEscapeReason` in `src/annotation.rs` |
| none | `NmdRuleId` in `src/annotation.rs` | MOVE |  | `NmdRuleId` in `src/annotation.rs` |
| none | `SoTerm` in `src/annotation.rs` | MOVE |  | `SoTerm` in `src/annotation.rs` |
| none | `TranscriptAccession` in `src/annotation.rs` | MOVE |  | `TranscriptAccession` in `src/annotation.rs` |
| none | `TranscriptConsequence` in `src/annotation.rs` | MOVE |  | `TranscriptConsequence` in `src/annotation.rs` |
| none | `ClassificationType` in `src/clinvar_value.rs` | MOVE |  | `ClassificationType` in `src/clinvar_value.rs` |
| none | `ClinVarValueError` in `src/clinvar_value.rs` | MOVE |  | `ClinVarValueError` in `src/clinvar_value.rs` |
| none | `CodingError` in `src/coding/mod.rs` | MOVE |  | `CodingError` in `src/coding/mod.rs` |
| none | `SoCode` in `src/coding/mod.rs` | MOVE |  | `SoCode` in `src/coding/mod.rs` |
| none | `ClingenDosageScoreCode` in `src/coding/vocab.rs` | MOVE |  | `ClingenDosageScoreCode` in `src/coding/vocab.rs` |
| none | `ClingenGeneValidity2017Code` in `src/coding/vocab.rs` | MOVE |  | `ClingenGeneValidity2017Code` in `src/coding/vocab.rs` |
| none | `ClinvarClassificationCode` in `src/coding/vocab.rs` | MOVE |  | `ClinvarClassificationCode` in `src/coding/vocab.rs` |
| none | `HpoInheritanceCode` in `src/coding/vocab.rs` | MOVE |  | `HpoInheritanceCode` in `src/coding/vocab.rs` |
| none | `AlleleKey` in `src/coordinate.rs` | MOVE | Hot path. | `AlleleKey` in `src/coordinate.rs` |
| none | `CoordinateError` in `src/coordinate.rs` | MOVE |  | `CoordinateError` in `src/coordinate.rs` |
| none | `CopyChange` in `src/copy_number.rs` | MOVE |  | `CopyChange` in `src/copy_number.rs` |
| none | `CopyNumberChange` in `src/copy_number.rs` | MOVE |  | `CopyNumberChange` in `src/copy_number.rs` |
| none | `CopyNumberCount` in `src/copy_number.rs` | MOVE |  | `CopyNumberCount` in `src/copy_number.rs` |
| none | `EditKind` in `src/edit.rs` | MOVE |  | `EditKind` in `src/edit.rs` |
| none | `GeneticCode` in `src/genetic_code.rs` | MOVE |  | `GeneticCode` in `src/genetic_code.rs` |
| none | `Change` in `src/hgvs/mod.rs` | MOVE | Hot path. Known conflict on enum tags stay unchanged. | `Change` in `src/hgvs/mod.rs` |
| none | `Hgvs` in `src/hgvs/mod.rs` | MOVE | Hot path (HGVS parser). | `Hgvs` in `src/hgvs/mod.rs` |
| none | `HgvsCode` in `src/hgvs/mod.rs` | MOVE | Hot path (HGVS parser). | `HgvsCode` in `src/hgvs/mod.rs` |
| none | `Inner` in `src/hgvs/mod.rs` | MOVE | Hot path (HGVS parser). | `Inner` in `src/hgvs/mod.rs` |
| none | `Reference` in `src/hgvs/mod.rs` | MOVE | Hot path (HGVS parser). | `Reference` in `src/hgvs/mod.rs` |
| none | `hgvs` in `src/hgvs/mod.rs` | MOVE | Hot path (HGVS parser). | `hgvs` in `src/hgvs/mod.rs` |
| none | `parse` in `src/hgvs/mod.rs` | MOVE | Hot path (HGVS parser). | `parse` in `src/hgvs/mod.rs` |
| none | `Anchor` in `src/hgvs/nucleotide.rs` | MOVE | Hot path (HGVS parser). | `Anchor` in `src/hgvs/nucleotide.rs` |
| none | `Bound` in `src/hgvs/nucleotide.rs` | MOVE | Distinct from the root `Bound`; it stays at `hgvs::Bound`. | `Bound` in `src/hgvs/nucleotide.rs` |
| none | `Location` in `src/hgvs/nucleotide.rs` | MOVE | Hot path (HGVS parser). | `Location` in `src/hgvs/nucleotide.rs` |
| none | `Molecule` in `src/hgvs/nucleotide.rs` | MOVE | Hot path (HGVS parser). | `Molecule` in `src/hgvs/nucleotide.rs` |
| none | `NaBody` in `src/hgvs/nucleotide.rs` | MOVE | Hot path (HGVS parser); enum tags stay unchanged. | `NaBody` in `src/hgvs/nucleotide.rs` |
| none | `NaChange` in `src/hgvs/nucleotide.rs` | MOVE | Hot path (HGVS parser). | `NaChange` in `src/hgvs/nucleotide.rs` |
| none | `NaCode` in `src/hgvs/nucleotide.rs` | MOVE | Hot path (HGVS parser). | `NaCode` in `src/hgvs/nucleotide.rs` |
| none | `NaEdit` in `src/hgvs/nucleotide.rs` | MOVE | Hot path (HGVS parser); enum tags stay unchanged. | `NaEdit` in `src/hgvs/nucleotide.rs` |
| none | `Position` in `src/hgvs/nucleotide.rs` | MOVE | Hot path. | `Position` in `src/hgvs/nucleotide.rs` |
| none | `PositionAnchor` in `src/hgvs/nucleotide.rs` | MOVE | Hot path (HGVS parser). | `PositionAnchor` in `src/hgvs/nucleotide.rs` |
| none | `Whole` in `src/hgvs/nucleotide.rs` | MOVE | Hot path (HGVS parser). | `Whole` in `src/hgvs/nucleotide.rs` |
| none | `Nucleotide` in `src/hgvs/nucleotide_value.rs` | MOVE | Hot path (HGVS parser). | `Nucleotide` in `src/hgvs/nucleotide_value.rs` |
| none | `Nucleotides` in `src/hgvs/nucleotide_value.rs` | MOVE | Hot path (HGVS parser). | `Nucleotides` in `src/hgvs/nucleotide_value.rs` |
| none | `RnaBase` in `src/hgvs/nucleotide_value.rs` | MOVE | Hot path (HGVS parser). | `RnaBase` in `src/hgvs/nucleotide_value.rs` |
| none | `Distance` in `src/hgvs/protein.rs` | MOVE | Hot path (HGVS parser). | `Distance` in `src/hgvs/protein.rs` |
| none | `Extension` in `src/hgvs/protein.rs` | MOVE | Hot path (HGVS parser); enum tags stay unchanged. | `Extension` in `src/hgvs/protein.rs` |
| none | `ProteinChange` in `src/hgvs/protein.rs` | MOVE | Hot path (HGVS parser); enum tags stay unchanged. | `ProteinChange` in `src/hgvs/protein.rs` |
| none | `ProteinCode` in `src/hgvs/protein.rs` | MOVE | Hot path (HGVS parser). | `ProteinCode` in `src/hgvs/protein.rs` |
| none | `ProteinRange` in `src/hgvs/protein.rs` | MOVE | Hot path (HGVS parser). | `ProteinRange` in `src/hgvs/protein.rs` |
| none | `ProteinSite` in `src/hgvs/protein.rs` | MOVE | Distinct from the root `ProteinSite`; it stays at `hgvs::ProteinSite`. | `ProteinSite` in `src/hgvs/protein.rs` |
| none | `CosmicKind` in `src/identifier.rs` | MOVE |  | `CosmicKind` in `src/identifier.rs` |
| none | `IdError` in `src/identifier.rs` | MOVE |  | `IdError` in `src/identifier.rs` |
| none | `ModeOfInheritance` in `src/inheritance.rs` | MOVE |  | `ModeOfInheritance` in `src/inheritance.rs` |
| none | `IscnCode` in `src/iscn/mod.rs` | MOVE |  | `IscnCode` in `src/iscn/mod.rs` |
| none | `iscn` in `src/iscn/mod.rs` | MOVE |  | `iscn` in `src/iscn/mod.rs` |
| none | `parse` in `src/iscn/mod.rs` | MOVE |  | `parse` in `src/iscn/mod.rs` |
| none | `Alternative` in `src/iscn/types.rs` | MOVE |  | `Alternative` in `src/iscn/types.rs` |
| none | `Arm` in `src/iscn/types.rs` | MOVE |  | `Arm` in `src/iscn/types.rs` |
| none | `Band` in `src/iscn/types.rs` | MOVE |  | `Band` in `src/iscn/types.rs` |
| none | `Breakpoint` in `src/iscn/types.rs` | MOVE |  | `Breakpoint` in `src/iscn/types.rs` |
| none | `Cells` in `src/iscn/types.rs` | MOVE |  | `Cells` in `src/iscn/types.rs` |
| none | `Chromosome` in `src/iscn/types.rs` | MOVE | Distinct from the root `Chromosome`; it stays at `iscn::Chromosome`. | `Chromosome` in `src/iscn/types.rs` |
| none | `Clone` in `src/iscn/types.rs` | MOVE |  | `Clone` in `src/iscn/types.rs` |
| none | `CloneBase` in `src/iscn/types.rs` | MOVE |  | `CloneBase` in `src/iscn/types.rs` |
| none | `Count` in `src/iscn/types.rs` | MOVE |  | `Count` in `src/iscn/types.rs` |
| none | `Derivative` in `src/iscn/types.rs` | MOVE |  | `Derivative` in `src/iscn/types.rs` |
| none | `Event` in `src/iscn/types.rs` | MOVE |  | `Event` in `src/iscn/types.rs` |
| none | `Item` in `src/iscn/types.rs` | MOVE |  | `Item` in `src/iscn/types.rs` |
| none | `Karyotype` in `src/iscn/types.rs` | MOVE |  | `Karyotype` in `src/iscn/types.rs` |
| none | `Mixture` in `src/iscn/types.rs` | MOVE |  | `Mixture` in `src/iscn/types.rs` |
| none | `Neo` in `src/iscn/types.rs` | MOVE |  | `Neo` in `src/iscn/types.rs` |
| none | `Part` in `src/iscn/types.rs` | MOVE |  | `Part` in `src/iscn/types.rs` |
| none | `Rearrangement` in `src/iscn/types.rs` | MOVE |  | `Rearrangement` in `src/iscn/types.rs` |
| none | `RearrangementKind` in `src/iscn/types.rs` | MOVE |  | `RearrangementKind` in `src/iscn/types.rs` |
| none | `SexChromosome` in `src/iscn/types.rs` | MOVE |  | `SexChromosome` in `src/iscn/types.rs` |
| none | `Sign` in `src/iscn/types.rs` | MOVE |  | `Sign` in `src/iscn/types.rs` |
| none | `Site` in `src/iscn/types.rs` | MOVE |  | `Site` in `src/iscn/types.rs` |
| none | `SiteKind` in `src/iscn/types.rs` | MOVE |  | `SiteKind` in `src/iscn/types.rs` |
| none | `LiftFailure` in `src/lift.rs` | MOVE |  | `LiftFailure` in `src/lift.rs` |
| none | `LiftOutcome` in `src/lift.rs` | MOVE |  | `LiftOutcome` in `src/lift.rs` |
| none | `Lifted` in `src/lift.rs` | MOVE |  | `Lifted` in `src/lift.rs` |
| none | `Code` in `src/outcome.rs` | MOVE |  | `Code` in `src/outcome.rs` |
| none | `InputCode` in `src/outcome.rs` | MOVE |  | `InputCode` in `src/outcome.rs` |
| none | `Outcome` in `src/outcome.rs` | MOVE |  | `Outcome` in `src/outcome.rs` |
| none | `Parsed` in `src/outcome.rs` | MOVE |  | `Parsed` in `src/outcome.rs` |
| none | `Refusal` in `src/outcome.rs` | MOVE |  | `Refusal` in `src/outcome.rs` |
| none | `Span` in `src/outcome.rs` | MOVE | Hot path. Distinct from `Span0`: byte offsets into source text. | `Span` in `src/outcome.rs` |
| none | `MAX_LEN` in `src/parse.rs` | MOVE |  | `MAX_LEN` in `src/parse.rs` |
| none | `parse_bytes` in `src/parse.rs` | MOVE |  | `parse_bytes` in `src/parse.rs` |
| none | `Frame` in `src/peptide.rs` | MOVE |  | `Frame` in `src/peptide.rs` |
| none | `PeptideError` in `src/peptide.rs` | MOVE |  | `PeptideError` in `src/peptide.rs` |
| none | `difference` in `src/peptide.rs` | MOVE |  | `difference` in `src/peptide.rs` |
| none | `peptide` in `src/peptide.rs` | MOVE |  | `peptide` in `src/peptide.rs` |
| none | `replacement` in `src/peptide.rs` | MOVE |  | `replacement` in `src/peptide.rs` |
| none | `site` in `src/peptide.rs` | MOVE |  | `site` in `src/peptide.rs` |
| none | `FrequencyError` in `src/population_frequency.rs` | MOVE |  | `FrequencyError` in `src/population_frequency.rs` |
| none | `ProteinSite` in `src/protein_site.rs` | MOVE |  | `ProteinSite` in `src/protein_site.rs` |
| none | `ProteinSiteError` in `src/protein_site.rs` | MOVE |  | `ProteinSiteError` in `src/protein_site.rs` |
| none | `TranscriptReference` in `src/protein_site.rs` | MOVE |  | `TranscriptReference` in `src/protein_site.rs` |
| none | `ReferenceError` in `src/reference.rs` | MOVE |  | `ReferenceError` in `src/reference.rs` |
| none | `parse_chromosome` in `src/reference.rs` | MOVE | Hot path. | `parse_chromosome` in `src/reference.rs` |
| none | `without_chr_prefix` in `src/reference.rs` | MOVE |  | `without_chr_prefix` in `src/reference.rs` |
| none | `Residue` in `src/residue.rs` | MOVE |  | `Residue` in `src/residue.rs` |
| none | `reverse_complement` in `src/sequence.rs` | MOVE |  | `reverse_complement` in `src/sequence.rs` |
| none | `trim_flanks` in `src/sequence.rs` | MOVE |  | `trim_flanks` in `src/sequence.rs` |
| none | `CohortObservation` in `src/source_shapes.rs` | MOVE |  | `CohortObservation` in `src/source_shapes.rs` |
| none | `CohortUnit` in `src/source_shapes.rs` | MOVE |  | `CohortUnit` in `src/source_shapes.rs` |
| none | `ConditionCount` in `src/source_shapes.rs` | MOVE |  | `ConditionCount` in `src/source_shapes.rs` |
| none | `Method` in `src/source_shapes.rs` | MOVE |  | `Method` in `src/source_shapes.rs` |
| none | `Subject` in `src/source_shapes.rs` | MOVE |  | `Subject` in `src/source_shapes.rs` |
| none | `SubjectError` in `src/source_shapes.rs` | MOVE |  | `SubjectError` in `src/source_shapes.rs` |
| none | `TranscriptId` in `src/source_shapes.rs` | MOVE |  | `TranscriptId` in `src/source_shapes.rs` |
| none | `SourceTextError` in `src/source_text.rs` | MOVE |  | `SourceTextError` in `src/source_text.rs` |
| none | `TextError` in `src/source_text.rs` | MOVE |  | `TextError` in `src/source_text.rs` |
| none | `AlignedExon` in `src/transcript.rs` | MOVE |  | `AlignedExon` in `src/transcript.rs` |
| none | `AlignmentError` in `src/transcript.rs` | MOVE |  | `AlignmentError` in `src/transcript.rs` |
| none | `Cigar` in `src/transcript.rs` | MOVE |  | `Cigar` in `src/transcript.rs` |
| none | `CigarCode` in `src/transcript.rs` | MOVE |  | `CigarCode` in `src/transcript.rs` |
| none | `CigarOp` in `src/transcript.rs` | MOVE |  | `CigarOp` in `src/transcript.rs` |
| none | `Transcript` in `src/transcript.rs` | MOVE |  | `Transcript` in `src/transcript.rs` |
| none | `TranscriptAlignment` in `src/transcript.rs` | MOVE |  | `TranscriptAlignment` in `src/transcript.rs` |
| none | `TranscriptError` in `src/transcript.rs` | MOVE |  | `TranscriptError` in `src/transcript.rs` |
| none | `TranscriptPosition` in `src/transcript_position.rs` | MOVE |  | `TranscriptPosition` in `src/transcript_position.rs` |
| none | `Region` in `src/transcript_region.rs` | MOVE |  | `Region` in `src/transcript_region.rs` |
| none | `RegionKind` in `src/transcript_region.rs` | MOVE |  | `RegionKind` in `src/transcript_region.rs` |
| none | `TranscriptRegion` in `src/transcript_region.rs` | MOVE |  | `TranscriptRegion` in `src/transcript_region.rs` |
| none | `TranscriptRegionKind` in `src/transcript_region.rs` | MOVE |  | `TranscriptRegionKind` in `src/transcript_region.rs` |
| none | `UncertainLocation` in `src/uncertain.rs` | MOVE |  | `UncertainLocation` in `src/uncertain.rs` |
| none | `AminoAcidsError` in `src/uniprot_feature.rs` | MOVE |  | `AminoAcidsError` in `src/uniprot_feature.rs` |
| none | `UniProtIsoformError` in `src/uniprot_feature.rs` | MOVE |  | `UniProtIsoformError` in `src/uniprot_feature.rs` |
| none | `AccessionCode` in `src/value.rs` | MOVE |  | `AccessionCode` in `src/value.rs` |
| none | `ExpressionError` in `src/variant/notation.rs` | MOVE |  | `ExpressionError` in `src/variant/notation.rs` |
| none | `Category` in `src/variant_reference.rs` | MOVE |  | `Category` in `src/variant/statement/category.rs` |
| none | `VariantRefError` in `src/variant_reference.rs` | MOVE |  | `VariantRefError` in `src/variant/statement/category.rs` |
| none | `Alternate` in `src/vcf.rs` | MOVE |  | `Alternate` in `src/vcf.rs` |
| none | `Site` in `src/vcf.rs` | MOVE |  | `Site` in `src/vcf.rs` |
| none | `SiteError` in `src/vcf.rs` | MOVE |  | `SiteError` in `src/vcf.rs` |
| none | `vcf` in `src/vcf.rs` | MOVE |  | `vcf` in `src/vcf.rs` |

## Approved API additions

The PM's 2026-11-01 ruling adds caller-safe macro implementations and compile-time checked Text literals. Hidden dependency re-exports let expansions use BioData's enabled dependencies when the caller renames BioData. Serde and schema gates are selected in BioData, not the invoking crate. Canonical enum schema names, identities and descriptions stay unchanged.

| Public item | Contract |
|---|---|
| `__biodata_text_serde` in `src/macros.rs` | Hidden serde expansion selected by BioData's feature. |
| `__biodata_text_schema` in `src/macros.rs` | Hidden schema expansion selected by BioData's feature. |
| `__biodata_enum_serde` in `src/macros.rs` | Hidden canonical enum serde expansion selected by BioData's feature. |
| `__biodata_enum_schema` in `src/macros.rs` | Hidden canonical enum schema expansion selected by BioData's feature. |
| `__biodata_wire_serde` in `src/macros.rs` | Hidden helper selected by BioData features for exported macro callers. |
| `__biodata_wire_schema` in `src/macros.rs` | Hidden helper selected by BioData features for exported macro callers. |
| `__biodata_native_serde` in `src/macros.rs` | Hidden helper selected by BioData features for exported macro callers. |
| `__biodata_native_schema` in `src/macros.rs` | Hidden helper selected by BioData features for exported macro callers. |
| `TextLiteral` in `src/text_literal.rs` | A checked static input token; its const constructor returns the same errors as Text's runtime constructor. Conversion to Text trims and allocates without a runtime refusal. |
| `text` in `src/text_literal.rs` | `text!(literal)` forces validation during const evaluation; invalid literals cannot compile. |
| `ProteinRangeError` in `src/hgvs/protein.rs` | Ticket 0024: `ProteinRange::new(first, last)` refuses a last position that does not lie after the first, with code `hgvs.protein_range.order`. One site is written through `ProteinRange::from(site)`. The HGVS parser checks range order through this constructor and still reports a disordered range as `hgvs.protein_edit.invalid`, so this code never reaches a parse result. The error type has no schema and is never serialized. |

## Approved author namespace removal

The PM's 2026-11-01 ruling removes the `semanticscholar:` alias from `AuthorId`. Only `semantic-scholar:` identifies that provider. The AuthorId pattern loses the alternate namespace and its schema description loses the obsolete acceptance clause; every reachable copy changes identically. This is one alias removal. Other existing schema roots, names and identities stay unchanged except where they embed this AuthorId definition.

Winnow 1.0.4 declares only `ascii` and `alloc`, with defaults off. The resolved `parser` feature comes from `ascii`; `std` stays disabled. The feature declaration changes no parser API or wire contract.

### Exported macro feature ownership

Every exported macro family uses BioData-selected helpers for serde and schema. `wire_enum!` preserves closed-vocabulary errors and described schemas; `native_enum!` preserves unknown publisher values. Featureless callers use a renamed dependency and instantiate `text_value!`, both forms of `wire_enum!`, `canonical_enum!`, `native_enum!`, `checked_id!` and `text!`, exercising their hidden helpers in all three dependency feature modes.

### Frozen additive operations

The shared values retain the additive operations present in the frozen API without changing their stored representation, constructors, existing parsers or serialized values. Scalar hashes follow existing equality, including `AccessionBase`'s versionless key. `Position1` converts through its existing checked constructor and keeps `CoordinateError`. `Expression` exposes its optional syntax version by borrow.

`Build`, `Strand`, `ManeStatus`, `ExpressionSyntax`, `RefSeqPrefix` and `EnsemblFeature` retain canonical value lists, conversions, ordering and hashing. Existing `Build::FromStr` keeps its typed error. The two prefix enums keep their current standalone snake-case serialization; their newly restored deserialization accepts only that wire form. Their newly restored `FromStr` accepts only canonical prefix text. These contexts stay distinct; no compatibility alias is added and no existing parser or output changes.

The eight closed coding enums retain their frozen wire-vocabulary operations; the actual open `native_enum!` family keeps its existing unknown-value representation and operations. `AminoAcids` and `UniProtIsoform` retain frozen additive text/ordering/hash operations while their existing constructors and typed conversion errors remain unchanged. Removed borrowed-text accessors on compact values and merged ClinVar type names remain deliberate consumer adaptations.

### Closed choice schema shape

`Category` and `RegionKind` use a plain string `enum`, replacing their derived `oneOf` of string `const` branches. Values and order stay unchanged. Each branch's description moves, unchanged and in the same order, to `x-enum-descriptions`, the existing vocabulary metadata convention. Root descriptions, titles, schema identities, serde behavior and parsing stay unchanged. The schema transform applies only to these two types. The exported-root guard rejects this malformed choice shape anywhere, including embedded definitions.

| Exported root | Approved changed schema nodes |
| --- | --- |
| `Category` | `/schema` |
| `RegionKind` | `/schema` |
| `Region` | `/schema/$defs/RegionKind` |
| `PopulationFrequency` | `/schema/$defs/Category`, `/schema/$defs/RegionKind` |
| `PredictorScore` | `/schema/$defs/Category`, `/schema/$defs/RegionKind` |
| `Statement` | `/schema/$defs/Category`, `/schema/$defs/RegionKind` |
| `Subject` | `/schema/$defs/Category`, `/schema/$defs/RegionKind` |
| `VariantRef` | `/schema/$defs/Category`, `/schema/$defs/RegionKind` |

The comparator permits exactly this metadata-preserving shape rewrite at those nodes, beside the approved AuthorId pattern and description removal. Every other schema byte and identity remains fixed.

### Object helper input boundary

Validated object values refuse positional JSON arrays before deserializing their fields. A shared generic serde map visitor delegates to each existing derived helper and validating conversion. Existing field validation, error codes, serialized output and schemas stay unchanged. Arrays inside object fields remain permitted.

The struct-helper values are `Span0`, `Bound`, `UncertainSpan0`, `GeneRef`, `Mane`, `TranscriptConsequence`, `Region`, `ProteinSite`, `TrialCoordinates`, `Allele`, `DrugTargetRelation`, `DrugInteractionClaim`, `SourceGeneRef`, `Expression`, `PopulationFrequency`, `ScientificPValue` and `ProfileExpression`. `Subject` applies the same boundary before its untagged gene and region wire helpers. `VariantRef`'s internally tagged helper already requires an object. Primitive text, numeric and collection values keep their own input shapes. The exported value macros deserialize primitive strings or enum names; their schema-only helper structs do not deserialize values.

The adjacently tagged enums `Coding`, `PathwayId`, `GtrTestRelation` and `DiagnosticRecord` also refuse positional arrays, which serde's generated helper previously accepted. Each enum and its private deserialization helper share one variant declaration. Their cases, serialized object fields and schemas stay unchanged. Plain directly derived public records retain their existing serde behavior. Non-human-readable deserializers retain the original helper path.


### Transcript string schema unions

`TranscriptReference` and `TranscriptId` serialize one JSON string. Their schemas use one string pattern alternating the exact `Accession` and `AccessionBase` patterns, in that order. The description names both forms. Schema identities, parsing and serialized values stay unchanged. The old `anyOf` branches and component definitions that become unreferenced disappear. Other component definitions remain unchanged.

The affected roots are `TranscriptReference` and `TranscriptId` at `/schema`; `ProteinSite`, `PopulationFrequency`, `Statement`, `Subject` and `VariantRef` at `/schema/$defs/TranscriptReference`; and `PredictorScore` at both `/schema/$defs/TranscriptReference` and `/schema/$defs/TranscriptId`. The comparator constructs exactly these rewrites from the frozen component patterns and permits only their exact field deltas and newly unreferenced `Accession`/`AccessionBase` definition removals. The exported-root guard rejects every `anyOf` or `oneOf` whose branches resolve to strings, including local references and nested string unions. Nullable or mixed-type unions remain valid.


### Subject level union

`Subject` emits one `oneOf` discriminated by the required `level` string constant. Its existing `VariantRef` branches are copied byte-identically and in the same order from their canonical generated definition. Gene and region branches reuse their existing object definitions with only `level` replaced by the corresponding direct string constant. Existing payload definitions stay unchanged and registered. The wire helpers, serde behavior, parsing, schema identity and public operations do not change.

Only `Subject` at `/schema` and its `Statement` embedding at `/schema/$defs/Subject` replace the old `anyOf` with this exact `oneOf`. The comparator constructs that exact replacement from frozen payload definitions. The exported-root guard rejects nonnullable untagged unions wrapping tagged unions, including references; ordinary nullable wrappers and field-presence constraints such as `Bound` and `DrugTargetRelation` remain unchanged.
