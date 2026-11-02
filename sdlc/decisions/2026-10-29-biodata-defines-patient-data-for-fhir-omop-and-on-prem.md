# BioData defines the patient data structures for FHIR, OMOP and on-premises work

Date: 2026-10-29

Decided by Ian.

## Ruling

Asked whether to run the experiment in which BioMCP fills the same BioData records from a FHIR server and an OMOP database and agents write scripts against those records, Ian said: "Can you go ahead and do these further experiments and help try to define what the BioData project is going to be? With regard to the data structures needed for FHIR and for OMOP and on-prem work."

Ian dictates. The transcript read "Fire" for FHIR and "Onmop" for OMOP.

## Reason

Ian's design has BioMCP reach FHIR servers and OMOP databases through adapters, BioData hold the results as structured Rust types, and agents work with those types in Python or Rust scripts. Earlier experiments tested parts of that design one at a time, and none filled the same BioData records from both backends. Patient data lives in software installed on premises at health systems, so the same records must serve it.

## What it means

- The PM runs the FHIR plus OMOP experiment as its own delegate. The same 100 MIMIC-IV demo patients are read from a FHIR server and from an OMOP database into one set of BioData records, every difference is named, and agents answer the same questions with scripts against raw FHIR, raw OMOP and the BioData records.
- The PM runs the private GENIE lung cohort experiment as planned.
- Milestone 3's value list comes from a design that covers FHIR, OMOP and patient data held on premises, tested by these experiments. Tickets for it still wait for the 1.0 cut.

## Options considered

- Leave milestone 3 undefined until after the 1.0 cut. Declined by the ruling.
- Run the full FHIR benchmark with the prototype getters. Ian declined it the same day.

## Replaces

Nothing. It narrows milestone 3's open questions.
