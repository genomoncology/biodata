# BioData milestone 3: patient-level values for FHIR, OMOP and on-premises data

Design draft, 2026-10-29. Status: ruled 2026-10-30. Milestone 3 builds its small values and none of its records ([ruling](../decisions/2026-10-30-clinical-milestone-builds-small-values-only.md)). It serves Ian's ruling in `sdlc/decisions/2026-10-29-biodata-defines-patient-data-for-fhir-omop-and-on-prem.md`. The lung cohort experiment and the records experiment test it. Nothing in it is built.

## 1. The split

BioData holds three things for patient data:

1. Small values that every format needs.
2. Thin fact records. Each record is built only from those values and from 1.0 values.
3. A source and a loss list on every record.

Consumers' adapters hold everything that knows a format. That covers FHIR JSON for each version and profile, SQL on FHIR views, OMOP SQL and vocabulary joins, and mapping source codes to standard concepts. It also covers vendor quirks, paging, bulk export, caches, identity crosswalks, cohort logic, deriving lines of therapy, imputing dates (such as the THEMIS end-date rule), time zone policy, and writers back to FHIR or OMOP.

The evidence supports this split:

- In the SDK experiment, converting through a shared model took 2.5 to 3 times the code of direct conversion (777 against 2,562 lines in round 3). The same model cut a query consumer from 138 lines to 48. It also caught 26 of 28 bad inputs and reported the other 2 as losses, where direct code caught 24. Its recommendation was to adopt the small values first and review records as a candidate contract.
- In the SQL on FHIR experiment, SQL on FHIR views passed 10 of 12 bad inputs silently. A typed boundary must check values whatever does the extraction.
- In the OHDSI experiment, a static scan found `source_concept_id` in 121 of 1,104 published phenotype SQL files and `type_concept_id` in 7. A coded value therefore keeps the standard, source and type concepts.
- In the FHIR getters experiment, named getters did not beat raw requests (86% against 90%). Records must keep every source element or name its loss.
- In the scripts comparison, BioMCP lost answers when it rewrote source text or turned a brand name into its ingredient. Records keep source text and codes verbatim. A standard concept sits beside the source and never replaces it.

**Readers and writers.** Milestone 3's text says the milestone delivers readers and writers. BioMCP's plan ("After 1.0") keeps resource shapes, views and query logic in its adapters. This design recommends adapters in consumers. The PM rules after the records experiment reports. If adapters stay in consumers, the done condition changes: BioData proves its records hold the first clinical slice, and BioMCP's release suite runs the round trip.

### Small values

| Value | Holds | Rules | Evidence |
|---|---|---|---|
| `TimePoint` | Date or date-time at year, month, day, second or fractional precision. Offset stated or unstated. Source text. | Reuses `PartialDate` for the date part. Never fills a missing part. Orders only at the coarser shared precision. | The SQL on FHIR experiment: 831,729 timestamps that `PartialDate` cannot hold. The clinical type study: 343 queries depend on precision, and offsets matter. In FHIR R4 a time requires an offset. OMOP datetimes carry none. |
| `TimeSpan` | Start and end `TimePoint`, either one absent | Start is no later than end | FHIR Period. OMOP start and end columns. |
| `Occurrence` | At a time, during a span, at an age, or described in text | | FHIR `onset[x]` and `effective[x]` |
| `Decimal` | Signed exact decimal with its scale | Keeps "1.50" apart from "1.5" | The SQL on FHIR experiment: 990 amounts differed as string and number. The SDK experiment flagged f64 as a gap. |
| `Unit` | Source text, UCUM code and OMOP unit concept, each optional | Never inferred. An absent unit stays absent. | The SQL on FHIR experiment: 101,210 quantities with no unit. BioMCP's `WithUnit` rule in `crates/biodata/src/number.rs`. |
| `Quantity`, `Range`, `ReferenceRange` | A decimal with comparator and unit. Low and high bounds. A reference range adds type and text. | Low is no greater than high when the units match. No unit conversion. | The SDK experiment's reversed-range probe |
| `ConceptCode` | System, code, version, display | Keeps the code's case. Checks syntax for known systems. Never checks that a code exists. | FHIR Coding. The SDK experiment round 3 ranked it first. |
| `CodeSystem` | Known systems, each with its FHIR URI and OMOP `vocabulary_id`. Any other URI. Any other vocabulary. Unstated. | A fixed table of system names. It never maps one code to another. | The SDK experiment: OMOP source fields lose their code system |
| `OmopConcept` | `concept_id`, plus optional name, domain, vocabulary, class, standard flag and code | Unmapped (0) and local (above 2,000,000,000) are named states | CDM 5.4 reserves IDs above 2 billion for site codes. The MIMIC-IV OMOP demo holds only local concepts. |
| `CodedValue` | Source text, source codes in order, OMOP source concept, OMOP standard concept | Never fills the standard concept from a source code. A second FHIR coding never proves a standard mapping. | The SQL on FHIR experiment's refusal `second_coding_does_not_prove_standard_mapping` |
| `Identifier` | Authority, value, kind | Authority is a URI, an OID, a named assigner or unstated. Comparison returns same, different or unknown. | The SQL on FHIR experiment: an identifier with no issuer passed silently |
| `RecordKey`, `RecordRef` | Origin label, resource type or table, native ID. A reference holds a key, an identifier and a display, each optional. | Keys are equal only within one origin. BioData never resolves a reference. | The SQL on FHIR experiment: dangling references passed silently |
| `PatientText` | Free text from a patient record | Prints redacted | Free text can hold names |

**Reuse.** The records reuse these 1.0 values:

- `PartialDate`, `Text`, `Attribution` and `Statement`
- `Curie` and `Concept`, so a condition can pass to BioMCP's disease and trial commands
- `GeneRef`, `HgncId`, `Variant`, `Expression`, `HgvsText` and the HGVS parser
- ISCN `Karyotype`, `Alteration`, `Signature` and `Origin`
- `Build`, `GenomicInterval` and `Allele`
- the ClinVar, dbSNP and COSMIC IDs

**Name collisions.** The merged crate already has these names (`sdlc/designs/2026-10-28-reconciliation.md`):

- `Coding`, a closed set of curated frameworks
- `Code`, a refusal code
- `Concept`, `Term` and `Origin`
- a nonnegative `Decimal` in the second consumer's `src/alteration/marker.rs`

The names above avoid all of them except `Decimal`. The PM decides whether the signed `Decimal` replaces the marker's type, which the second consumer would measure, or takes another name.

**Feature flag.** All new code sits in a `clinical` module behind a `clinical` feature. The second consumer's hot path then compiles none of it.

### Records

Every record has these fields:

- `source`
- `losses`
- `subject`, a reference to the patient (absent on Patient)
- `status`, the verbatim source status as a `ConceptCode` where the format has one
- `links`, a list of `{relation, target}` with relations about, part of, has member, derived from, reason, based on, specimen and report

Links carry FHIR `focus`, `partOf`, `hasMember`, `derivedFrom` and `Condition.stage.assessment`. They also carry OMOP's `measurement_event_id` with `meas_event_field_concept_id`, and `EPISODE_EVENT`. The clinical type study named self-referencing links as a test any model must pass.

The records are Patient, VitalStatus, Condition, Observation, Medication, Procedure, Specimen, Encounter, GenomicFinding, Report and Episode.

mCODE STU4 fixes which facts matter. Its six groups are patient, disease, assessment, genomics, treatment and outcome. mCODE profiles stage, tumor markers, performance status and disease status as Observations, so this design carries them as Observation records linked to the condition. A clinical reviewer confirms the list before the record tickets start.

## 2. Records: where each field comes from and what each mapping loses

Each table lists a record's fields, the FHIR R4 elements and the OMOP 5.4 columns. The text under each table names the losses. Section 5 lists what the on-premises engine supplies.

**Patient**

| Field | FHIR R4 | OMOP 5.4 |
|---|---|---|
| identifiers | `identifier` | `person_source_value`; `person_id` is the key |
| birth | `birthDate` | `year_of_birth`, `month_of_birth`, `day_of_birth`, `birth_datetime` |
| administrative_gender | `gender` | none |
| sex_at_birth | US Core birthsex extension | `gender_concept_id`, `gender_source_value` |
| race, ethnicity | US Core extensions | `race_*`, `ethnicity_*` |

OMOP's `person_source_value` has no authority. The CDM documents `gender_concept_id` as "biological sex at birth", so FHIR `gender` never fills `sex_at_birth`. OMOP holds one race concept, and FHIR's extension holds several categories. The record holds no names, contact details, addresses or postal codes. Each one present becomes a dropped loss with reason `direct_identifier_not_held`. The PM rules on that choice.

**VitalStatus**

| Field | FHIR R4 | OMOP 5.4 |
|---|---|---|
| status: deceased, stated alive, unstated | `deceased[x]` | whether a DEATH row exists |
| date | `deceasedDateTime` | `death_date`, `death_datetime` |
| cause | none on Patient | `cause_concept_id`, `cause_source_value`, `cause_source_concept_id` |

A person with no DEATH row is unstated, never alive. `deceasedBoolean` false means stated alive, with no date. mCODE's CancerPatient profile marks `deceased[x]` must-support. FHIR R4 Patient carries no cause of death.

**Condition**

| Field | FHIR R4 | OMOP 5.4 |
|---|---|---|
| code | `code` | `condition_concept_id`, `condition_source_value`, `condition_source_concept_id` |
| body_sites, histology | `bodySite`; mCODE laterality and histology extensions | inside one ICD-O-3 concept |
| clinical_status, verification_status | `clinicalStatus`, `verificationStatus` | none |
| diagnosis_role | `Encounter.diagnosis.use` | `condition_status_concept_id` |
| onset, abatement | `onset[x]`, `abatement[x]` | `condition_start_*`, `condition_end_*` |
| recorded, encounter | `recordedDate`, `encounter` | none, `visit_occurrence_id` |

OMOP has no clinical or verification status. Its `condition_status_concept_id` states the diagnosis role in a visit, so it fills `diagnosis_role` instead. An onset given as an age, a range or text has no OMOP column. OMOP's oncology conventions record a cancer as one ICD-O-3 concept, and splitting it into site and histology is a vocabulary join in the adapter. Stage arrives as linked Observation records in both formats. FHIR links them through `Condition.stage.assessment`. OMOP links them through `meas_event_field_concept_id` 1147127 and `measurement_event_id`.

**Observation**

| Field | FHIR R4 | OMOP 5.4 |
|---|---|---|
| code | `code` | `measurement_concept_id` or `observation_concept_id`, with source columns |
| value | `value[x]` | `value_as_number`, `value_as_concept_id`, `value_as_string`, `value_source_value`, `operator_concept_id` |
| unit | `valueQuantity` unit, system, code | `unit_concept_id`, `unit_source_value`, `unit_source_concept_id` |
| reference_ranges | `referenceRange` | `range_low`, `range_high` |
| interpretation, issued | `interpretation`, `issued` | none |
| effective | `effective[x]` | `measurement_date`, `measurement_datetime`, `measurement_time` |
| components | `component` | separate rows, linked by `measurement_event_id` |

OMOP holds one numeric range. It has no interpretation, issued time or specimen link, and no Ratio, Period or SampledData value. A group of FHIR components becomes separate rows, and the grouping survives only through `measurement_event_id`.

**Medication.** One record covers orders and administrations. Its `kind` is order, administration, dispense, statement or unknown. OMOP keeps all drug rows in one table, and the SDK experiment found that an unknown `drug_type_concept_id` cannot honestly become either an order or an administration.

| Field | FHIR R4 | OMOP 5.4 |
|---|---|---|
| kind | resource type | `drug_type_concept_id` |
| drug | `medication[x]` | `drug_concept_id`, `drug_source_value`, `drug_source_concept_id` |
| status, status_reason | `status`, `statusReason` | `stop_reason` |
| intent, reasons, treatment_intent | `intent`, `reasonCode`/`reasonReference`, mCODE treatmentIntent extension | none |
| period, authored | `MedicationAdministration.effective[x]`; `authoredOn` and dosage or dispense periods | `drug_exposure_start_*`, `drug_exposure_end_*`, `verbatim_end_date` |
| dose, route, sig, supply | dosage, `dispenseRequest` | `quantity`, `route_concept_id`, `sig`, `days_supply`, `refills` |
| order | `MedicationAdministration.request` | none |

OMOP requires an end date. When the source has none, the adapter infers one by a named rule and records an inferred loss. OMOP drops intent, reasons, treatment intent and the link from an administration to its order. A Medication resource's ingredients and strengths become a dropped loss. The drug keeps its source code beside its standard concept.

**Procedure**

| Field | FHIR R4 | OMOP 5.4 |
|---|---|---|
| code | `code` | `procedure_concept_id` with source columns |
| performed | `performed[x]` | `procedure_date`, `procedure_datetime`, `procedure_end_date`, `procedure_end_datetime` |
| status, body_sites, reasons, intent | `status`, `bodySite`, `reasonCode`/`reasonReference`, mCODE procedureIntent extension | none |
| modifier, quantity | none | `modifier_concept_id`, `quantity` |

OMOP keeps no status, body site, reason or intent, and CDM 5.3 keeps no end date. mCODE's RadiotherapyCourseSummary carries modality, technique, sessions and dose. OMOP core has no columns for them, so they stay dropped losses until a reviewed convention exists.

**Specimen**

| Field | FHIR R4 | OMOP 5.4 |
|---|---|---|
| type | `type` | `specimen_concept_id`, `specimen_source_value` |
| identifiers | `identifier`, `accessionIdentifier` | `specimen_source_id` |
| collected, body_site, quantity | `collection.*` | `specimen_date`/`_datetime`, `anatomic_site_*`, `quantity`, `unit_*` |
| disease_status | none | `disease_status_concept_id`, `disease_status_source_value` |
| parent | `parent` | none |

FHIR R4 has no disease status, and OMOP has no parent specimen or processing. No OMOP column links a measurement to its specimen, so a genomic finding loses its specimen link in OMOP.

**Encounter**

| Field | FHIR R4 | OMOP 5.4 |
|---|---|---|
| classes, types | `class`, `type` | `visit_concept_id`, `visit_source_value` |
| period, status | `period`, `status` | `visit_start_*`, `visit_end_*`; no status |
| admitted_from, discharged_to | `hospitalization.admitSource`, `dischargeDisposition` | `admitted_from_*`, `discharged_to_*` |
| stays (location, period) | `location` with its `period` | VISIT_DETAIL rows |
| part_of | `partOf` | `visit_occurrence_id` and `parent_visit_detail_id` on VISIT_DETAIL |

FHIR location stays keep their order and periods, which the clinical type study named as a test. OMOP keeps stays only when its ETL filled VISIT_DETAIL.

**GenomicFinding**

| Field | FHIR R4 (Genomics Reporting STU3 Variant) | OMOP 5.4 |
|---|---|---|
| presence: present, absent, indeterminate, no call | `value`, LOINC answer list LL1971-2 | no settled convention |
| genes | gene-studied component | The gene of an OMOP Genomic concept |
| variant (c., g., p., transcript, reference) | 48004-6, 81290-9, 48005-3, 51958-7, 48013-7 | OMOP Genomic `measurement_concept_id`; HGVS in `value_source_value`, at most 50 characters |
| cytogenomic | 81291-7 (ISCN) | none |
| location, ref and alt alleles | exact-start-end, coordinate-system, ref-allele, alt-allele | none |
| origin, allelic_state | 48002-0, 53034-5 | none |
| allele_fraction, read_depth, copy_number | 81258-6, 82121-5, 82155-3 | at most `value_as_number`, by local convention |
| variation_ids, confidence | 81252-9, variant-confidence-status | none |
| signature (TMB, MSI) | Genomics Reporting biomarker profiles | a MEASUREMENT row |

The OMOP row loses the protein and genomic forms, the location, the origin and the specimen link. An absent finding cannot be told apart from a missing row. OHDSI's Genomic CDM of 2019 is a prototype, and its forum still records variants as MEASUREMENT rows with no agreed value convention and no copy number concepts.

mCODE records a pertinent negative as one GenomicVariant with value Absent and every negative gene listed. BioData parses each HGVS expression with the merged parser and keeps its parsed or refused outcome. Implications from the report map to BioData's existing `Statement`, linked to the Report.

**Report**

| Field | FHIR R4 | OMOP 5.4 |
|---|---|---|
| code, status, issued, effective | DiagnosticReport fields | none; NOTE holds text |
| performer | `performer` | `provider_id`, `care_site_id` |
| results, specimens | `result`, `specimen` | none |
| basis (knowledge versions) | none | none |
| presented_form | `presentedForm` | NOTE text |

OMOP has no report table. BioData holds a reference to an attached PDF and never its bytes.

**Episode**

| Field | FHIR R4 | OMOP 5.4 |
|---|---|---|
| kind, object (regimen or disease phase) | none in core FHIR or mCODE STU4 | `episode_concept_id`, `episode_object_concept_id` |
| number, parent | none | `episode_number`, `episode_parent_id` |
| period, members | CarePlan when a source uses one | `episode_start_*`, `episode_end_*`, EPISODE_EVENT |

mCODE STU4 lists no regimen or line-of-therapy profile. An Episode exists only when a source states one. BioData never derives a regimen or a line of therapy from drug rows. A clinical reviewer defines both, and any derivation lives in a consumer with its cited rule. No public synthetic data holds CDM 5.4 Episode rows.

### How a record shows its source and losses

`RecordSource` has these fields:

- `format`: `fhir-r4`, `fhir-r4b`, `fhir-r5`, `fhir-r6`, `omop-cdm-5.3`, `omop-cdm-5.4` or other text
- `profiles`: claimed canonical URLs
- `origin`: a label the consumer assigns to a server or database, never a URL with credentials
- `key`
- `updated`: FHIR `meta.lastUpdated`
- `record_type`: OMOP's `*_type_concept_id`
- `coverage`: complete when the adapter walked every element, partial when it read chosen paths (a view is always partial)

A `Loss` has `at` (the source path), `kind` and `reason` (a stable code). The kinds are:

- dropped: no field holds the element
- changed: the precision or form changed
- inferred: a named rule supplied the value
- contextual: configuration supplied a fact, such as an identifier's authority
- refused: the value failed a check and stays out, and the record stays

A loss never holds the value. A writer returns its own loss list for the target format.

```json
{"kind": "patient",
 "identifiers": [{"authority": {"uri": "http://hospital.example.org/mrn"}, "value": "000123", "kind": "MR"}],
 "birth": {"text": "1950-04", "precision": "month"},
 "source": {"format": "fhir-r4",
            "profiles": ["http://hl7.org/fhir/us/mcode/StructureDefinition/mcode-cancer-patient"],
            "origin": "fhir-a", "key": {"kind": "Patient", "id": "p1"}, "coverage": "complete"},
 "losses": [{"at": "Patient.name", "kind": "dropped", "reason": "direct_identifier_not_held"}]}
```

## 3. Identity and linking

**One backend.** `RecordKey` identifies a record within one backend. Origin labels keep `Patient/1` on two servers apart.

**The same person in two backends.** Two `Identifier` values are the same only when both authorities are stated and equal and the values are equal. An unstated authority gives unknown. OMOP's `person_source_value` has no authority, so the adapter attaches one from configuration and records a contextual loss. The SDK experiment treated that kind of configuration as outside context.

When no shared identifier exists, the site's own crosswalk links persons. The consumer states each link as a `PersonLink {left, right, basis}`, where the basis is a shared identifier or a named crosswalk. BioData never matches on names, birth dates or addresses. Probabilistic matching belongs to a master patient index.

**The same fact in two backends.** FHIR and OMOP share no fact key. After the records experiment reports, BioData may add `differences(a, b)` for each record. It would list field differences under fixed rules:

- Times compare at the coarser shared precision.
- Codes match on the same system and code, or on the same OMOP standard concept.
- Quantities match on the same unit code and an equal decimal, with no unit conversion.

The consumer decides when the remaining differences still make one fact. The records experiment should show which key works.

## 4. Versions

Records name no FHIR or OMOP version, and `RecordSource.format` says which version filled each record. Adapters absorb these shape changes:

| Change | Source |
|---|---|
| R4 `Procedure.performed[x]` became `occurrence[x]` in R5 | R5 Procedure |
| `MedicationAdministration.effective[x]` became `occurence[x]` in R5 and `occurrence[x]` in R6 | R5 page and R6 build |
| `Encounter.period` became `actualPeriod`, `hospitalization` became `admission`, `class` became a list, and `statusHistory` was deleted | R6 build, R4-to-R6 list |
| Condition's recorder and asserter moved into `participant` in R5 and return in R6. `clinicalStatus` went from 0..1 to 1..1 to 0..1. | R5 page and R6 build |
| R5 split `MedicationRequest.reported[x]` into `reported` and `informationSource`, and the R6 build removes `reported` | R5 page and R6 build |
| `DiagnosticReport.imagingStudy` became `study`, and `issued` changes from instant to dateTime in R6 | R6 build |
| VISIT_OCCURRENCE `admitting_source_concept_id` became `admitted_from_concept_id` in 5.4 | CDM 5.4 changes |
| 5.4 adds `procedure_end_date`, `measurement_event_id`, EPISODE and EPISODE_EVENT | CDM 5.4 changes |

When a 5.3 source lacks a 5.4 field, the field stays absent with no loss.

**Status.** Status value sets change across versions. R5 replaced Encounter's status set and added "ended" to MedicationRequest. Records therefore keep the verbatim status with its code system. A normalized status waits for the records experiment's evidence.

**Profiles.** Profiles stay in adapters too. mCODE STU4 builds on US Core 6.1 while US Core stands at 9.0.0. mCODE's genomic profiles derive from Genomics Reporting STU2 while STU3 is current, so two Genomics Reporting versions are live inside R4.

**Field rule.** A record gains a field in three cases: FHIR R4 and OMOP 5.4 both carry the fact, mCODE or Genomics Reporting profiles it, or the on-premises engine supplies it. A fact that only one version carries stays a loss until a ticket adds it.

**R6.** The R6 build is `6.0.0-snapshot1`, after five ballots. It marks Patient, Condition, Observation, MedicationRequest, MedicationAdministration, Procedure, Specimen, DiagnosticReport and Encounter normative. US Core plans R6 support from its version 11 at the earliest. The first adapters target R4.

## 5. On-premises needs

The on-premises engine is described in an internal architecture note. It reads VCF, BAM and QC files and integrates with LIS and LIMS over HL7, REST, SOAP or FTP. It sends signed reports to the EMR, and it encrypts patient fields at rest.

| The engine holds or sends | BioData value or record |
|---|---|
| Name, MRN, date of birth, postal code | `Patient.identifiers` (MRN) and `Patient.birth`; names and postal codes are not held |
| Tumor type, sex and age used for trial matching | Condition, Patient |
| Sample and run QC, pass or fail | Specimen, with linked Observations |
| Alterations with HGVS, HGNC gene, allele frequency, read depth and reference build | GenomicFinding |
| Therapy assertions and trial matches | existing `Statement` and `NctId`, linked to the Report |
| Signed final report and its sign-off | Report status and issued time; the PDF as a reference only |
| Annotation and actionability versions for each order | `Report.basis` as `Attribution` values |
| Audit log of who viewed patient data | stays in the engine |

The engine's own adapter lives in its private code.

**Printing rule.** BioData writes no logs or caches. Its leak paths are Debug output, Display output, error messages and panics. The workspace denies missing Debug implementations, and the reconciliation proposes forbidding them, so every record has Debug. A derived Debug prints every field. This design proposes the following rule:

- **Sensitive leaves print their shape.** `Identifier` prints its authority and the value's length. `RecordKey` prints its origin, its kind and the ID's length. `TimePoint` prints its precision and whether an offset is stated. `PatientText` prints its length. HIPAA's Safe Harbor method lists names, record numbers and every element of a date except the year as identifiers (45 CFR 164.514(b)(2)). This rule also hides the year, because Safe Harbor counts ages over 89 as identifiers too.
- **Records derive Debug.** Every record field is a BioData leaf, so a derived Debug prints only redacted forms. A gate test fills every record with sentinel strings and dates, and it fails when Debug output contains one. The second consumer already prints a source's length and never its text (`crates/biodata/src/outcome.rs`).
- **Sensitive leaves implement Display only through an explicit view.** Code that writes a value out calls `id.expose()`. Serde writes real values, because serialization is deliberate output. JSON Schema marks sensitive fields `x-biodata-sensitive: true` so that consumers keep them out of caches.
- **Errors name a field path and a stable code.** They keep no input, as the carried rule already says.
- **Fixtures are synthetic.** Synthea (Apache-2.0) and the HL7 examples (CC0) qualify. The MIMIC-IV demo is licensed ODbL, and its share-alike condition goes beyond a notice, so under `AGENTS.md` it stays out of the repository.

## 6. Python and TypeScript

Python sees immutable classes backed by the Rust constructors, as the SDK experiment recommended. The distribution name is still open (idea 0003).

```python
obs = Observation.from_json(text)       # raises Refused(code, path)
obs.code.source[0].system               # 'http://loinc.org'
obs.value.quantity.value                # Decimal('1.20')
obs.value.quantity.unit.ucum            # None when the source stated no unit
obs.effective.at.precision              # TimePrecision.SECOND
obs.source.format, [l.reason for l in obs.losses]
repr(patient.identifiers[0])            # Identifier(authority='http://…/mrn', value=<6 chars>)
patient.identifiers[0].expose()         # '000123'
```

TypeScript sees readonly types generated from the JSON Schema, with discriminated unions such as `value.kind === "quantity"`. A WebAssembly build of the Rust checks provides the factories, because a schema alone does not check calendars, HGVS or field pairs (the SDK experiment). Decimals arrive as strings so they keep their scale. Inspect output is redacted. Milestone 4 builds both SDKs. Milestone 3 ships the schemas they need.

## 7. Open questions

| Question | Answered by |
|---|---|
| Do readers and writers live in BioData or in adapters? | The records experiment's adapter line counts and what a third backend reuses; the lung cohort experiment's third format |
| Do the same 100 patients give the same records from both backends, and what causes each difference? | The records experiment, step 2 |
| Does one script run unchanged on both backends and answer as well as raw access? | The records experiment, step 3, against the FHIR getters experiment's baseline |
| Does MIMIC's FHIR patient identifier match the OMOP demo's `person_source_value`? | The records experiment, step 2 |
| Which key makes two records one fact? Do scripts need a normalized status? | The records experiment's comparison and scripts |
| Do decimals go on the wire as strings or as JSON numbers? | The records experiment's Python loading; the SQL on FHIR experiment's 990 amounts |
| How does a time with no offset compare with one that has an offset? | The records experiment, with MIMIC's time-shifted timestamps |
| Which facts does each record need for a cancer care decision, and in what order should they be built? | Clinical reviewer, starting from mCODE STU4 and the 47 report objects (replication corpus row 13) |
| What are a regimen and a line of therapy, and does BioData hold any derived episode? | Clinical reviewer; the lung cohort experiment follows the paper's first-line rule |
| Which dates does survival need: death, last known alive, or the OMOP observation period? | The lung cohort experiment's 17.4-month target; clinical reviewer |
| How does a genomic finding enter OMOP without losing long HGVS or absent results? | OHDSI's Genomic subgroup; a synthetic 5.4 fixture |
| May public fixtures carry SNOMED CT codes from HL7 examples and Synthea? | The PM checks the SNOMED CT terms the FHIR license page names, and asks Ian if it needs a license judgment |
| Does Patient hold names, contact details or postal codes? | PM ruling; this design proposes no |

## 8. Tickets for milestone 3, after the 1.0 cut

Each ticket has one outcome.

1. A gap map places the mCODE STU4 and Genomics Reporting STU3 examples and the 47 report objects onto this design, and lists each gap (idea 0005).
2. A decision records the split ruling from the lung cohort and records experiments' reports and restates the milestone's done condition.
3. The gate fails when any record's Debug output shows a sentinel value.
4. `TimePoint`, `TimeSpan` and `Occurrence` hold every date and time form in the SQL on FHIR experiment's fixtures and the MIMIC FHIR demo, with precision and offset kept.
5. `Decimal`, `Unit`, `Quantity`, `Range` and `ReferenceRange` keep decimal scale, and unitless quantities stay unitless.
6. `ConceptCode`, `CodeSystem`, `OmopConcept` and `CodedValue` hold paired source and standard codes, with unmapped and local concepts as named states.
7. `Identifier`, `RecordKey`, `RecordRef` and `PersonLink` compare identity with three outcomes.
8. `RecordSource`, `Loss` and `Link` appear on a first record and in its JSON Schema.
9. Patient and VitalStatus.
10. Condition and Observation, with stage and tumor markers as linked Observations.
11. Medication, with its five kinds.
12. Procedure, Specimen and Encounter.
13. GenomicFinding and Report, on the merged HGVS, ISCN and alteration values.
14. Episode, after the clinical reviewer rules on regimens and lines of therapy.
15. The first clinical slice fills every record from Synthea R4 and Eunomia lung (CDM 5.3) through BioMCP's adapters, round-trips, and reports its losses. This meets the milestone's done condition.

Tickets 4 to 7 can run in parallel. Tickets 9 to 13 wait on ticket 8 and on ticket 2's ruling.
