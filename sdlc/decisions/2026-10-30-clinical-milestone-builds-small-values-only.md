# The clinical milestone builds the small values only

Date: 2026-10-30

Decided by the BioData PM, under milestone 3's own rule and Ian's 2026-10-29 ruling that BioData defines the patient data structures for FHIR, OMOP and on-premises work.

## Ruling

Milestone 3 builds the small shared values only. BioData does not ship thin clinical records, and it ships no FHIR or OMOP readers or writers. Consumers' adapters build their own records from BioData's values and report what each mapping keeps, changes and loses.

## Reason

Milestone 3 said it builds the records only if one record model fits both backends, agents' scripts against it run unchanged on both, and the shared values cost less for each added format. Two experiments tested that, and neither met it.

- The records experiment filled one set of draft records for the same 100 MIMIC-IV demo patients from a FHIR server and from an OMOP database. The patients and their identifiers matched, but the records did not. Every agent script ran unchanged on the other backend, and only 17 of 48 gave the same answer there. Scripts against the records answered as well as raw access on the same backend and cost about 1.6 times the tokens, mostly in record output.
- The lung cohort experiment read the public GENIE BPC NSCLC release privately through a draft record model and, in a second blind arm, directly. Both arms wrote FHIR and OMOP and reproduced the paper's median survival from each output. The draft model refused fewer of 32 altered bad inputs (24 against 27), and its next output format costs more (about 300 lines against 220).

In both experiments the small values held: a time with its precision, a decimal kept as text with its scale, a unit and quantity, a coded value with its source beside its standard concept, an identifier with its authority, a record source and a loss with its kind. The records each experiment needed were shaped by its source. The two adapters in the records experiment shared only a few tens of lines of helpers.

## What it means

- The value list for milestone 3 is: TimePoint, TimeSpan, Decimal, Unit, Quantity, Range, ReferenceRange, ConceptCode, CodeSystem, OmopConcept, CodedValue, Identifier, RecordKey, RecordRef, RecordSource, Loss and its kinds, and PatientText for sensitive leaves. Fixes the experiments found join it:
  - TimePoint takes a day offset from an anchor such as birth, and anchoring it to a calendar date is a loss.
  - Decimals compare by value, and a source that stores floats records that the scale is unknown.
  - Text values order, and partial dates support date arithmetic.
  - A coding is an open system and code pair.
  - The small helpers both adapters repeated (key, number and time parsing, the source block and the dropped-field loss) are candidate BioData functions.
- Record shapes stay in consumers. The experiments' record findings (encounter kind, observation stay, one microbiology shape, medication kind, last-known-alive, condition role and sequence, episode trial flag and line, report to condition link) go to consumers as guidance, not into BioData.
- Milestone 3 is done when the replication set's first clinical slice round-trips through FHIR and OMOP in a consumer's adapter built on these values, with its losses reported.
- The patient data design stays as reference for the values and the on-premises rule; its record sections are not built.
- Ian can overturn this. A later experiment that shows a shared record pays off from three formats on would reopen it.
