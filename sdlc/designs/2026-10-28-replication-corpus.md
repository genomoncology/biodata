# Replication corpus

Study, 2026-10-28. It lists the public and synthetic datasets, benchmarks and published results that BioData's values and BioMCP's commands should reproduce, and it says who runs each and when. It serves milestone 2 and Ian's rule that the first users are datasets, experiments and papers we try to replicate. Metadata only: nothing here was downloaded.

## Who runs what

BioMCP owns its crate and commands until the 1.0 cut, and the second consumer owns the HGVS parser that becomes BioData's base. So a run that can catch a wrong 1.0 value before the cut goes to the owning team now. After the merge, the parse corpora move into BioData's release suite. FHIR and OMOP runs wait for 1.1.

Every fixture records its origin, license and checksum. Fixtures in this repository are synthetic, or come from sources whose license allows republishing with no condition beyond keeping a notice, such as public-domain, CC0, MIT and Apache-2.0 sources. Licensed or controlled-access data, and cases derived from it or from copyrighted references, never enter this repository, even as excerpts. Large inputs live under the data root, never in Git.

Every other dataset here is read in place by private tests and experiments outside this repository. Those runs report only counts, and their findings come back as tickets.

## Before the cut: BioMCP's values and commands

1. **cBioPortal's agent question bank.** https://github.com/cBioPortal/cbioportal-mcp-qa, Apache-2.0, updated 2026-10-26. It holds 146 graded questions in four tracks, 62 of them numeric data questions. On 2026-10-15 cBioPortal's own agent passed 74% overall and 82% of data questions. Target: run the 62 data questions through BioMCP against 82%. Failures show cohort mistakes, such as sample against patient denominators or unprofiled samples. This is the direct test of covering what cBioPortal does.
2. **CIViC monthly release.** https://civicdb.org, CC0, released on the 1st of each month. On 2026-10-28 the API reported 11,743 evidence items, 5,168 variants and 5,777 molecular profiles. Target: every variant, molecular profile, disease, drug and citation in the TSV parses into a 1.0 value or is refused with a named reason, and the counts per gene match the TSV.
3. **ClinVar and ClinVar-GKS.** https://dataexchange.clinicalgenome.org/clinvar-gks/, free monthly and weekly JSON and Parquet with VRS identities. Target: every variant's identifiers and HGVS text parse into 1.0 values, and the printed variant IDs match ClinVar's own.
4. **TREC Precision Medicine 2017 to 2019 and TREC Clinical Trials 2021 to 2023.** https://trec.nist.gov/data/precmed2019.html and https://trec.nist.gov/data/trials2022.html, open. Topics name cancer type, genes and demographics, with judged PubMed and ClinicalTrials.gov results. Target: the published trec_eval scores. Score only against the judged trial snapshot, because live trials have changed.
5. **TrialGPT.** https://github.com/ncbi-nlp/TrialGPT, open code and data, 183 synthetic patients. Target: 87.3% criterion accuracy on 1,015 patient-criterion pairs.
6. **MCP-Bench and InfoMosaic-Bench.** arXiv 2508.20453 and 2510.02271. Both already include an old BioMCP. On InfoMosaic's 83 biology questions, GPT-5 scored 53% with web search and 43% with BioMCP 0.4.6 added. Target: a rerun with BioMCP 1.0 that beats web search alone.
7. **MedBrowseComp.** arXiv 2505.14963, open. 605 deep-research questions built from 121 trials over PubMed, ClinicalTrials.gov, FDA and HemOnc.org. Frontier agents score as low as about 10%.
8. **Trial criteria the registry hides.** The KarMMa-1 real-world study (Br J Haematol 2024, PMID 38263627) applied the trial's criteria to 69 patients. The registry says only "Inadequate organ function", and the thresholds appear only in the protocol. Target: BioMCP's trial values hold a criterion as a threshold with its timepoint and exceptions.

## Before the cut: the HGVS parser

9. **biocommons hgvs test data.** https://github.com/biocommons/hgvs, Apache-2.0. It ships grammar_test.tsv, reject cases and ClinVar samples. Target: the same accept and reject result on every row.
10. **Yen and others 2017, "A variant by any name".** https://pmc.ncbi.nlm.nih.gov/articles/PMC5267466/. 126 curated variants. Target: compare our parse and projection with the published SnpEff (89.8%) and VEP (82.2%) accuracy on coding substitutions.
11. **GA4GH VRS validation suite.** https://github.com/ga4gh/vrs/tree/main/validation, Apache-2.0, VRS 2.1.1. Target: every fixture whose inputs BioData can hold yields the expected value.

## After the cut: BioData 1.1 clinical values

12. **mCODE STU4 and Genomics Reporting STU3 examples.** https://hl7.org/fhir/us/mcode/examples_genomics.html and https://build.fhir.org/ig/HL7/genomics-reporting/downloads.html. They cover an indel, copy number changes, fusions, a pertinent negative, TMB, MSI, specimens and therapeutic implications. Target: each example round-trips with every lost field reported.
13. **Somatic report adequacy, JAMIA Open 2026.** https://academic.oup.com/jamiaopen/article/doi/10.1093/jamiaopen/ooag022/8508642. Twenty public somatic reports held 47 semantic objects, and 41 encode in FHIR Genomics. Target: BioData holds at least 41 of 47.
14. **MIMIC-IV FHIR demo with FHIR-AgentBench and FHIRPath-QA.** https://physionet.org/content/mimic-iv-fhir-demo/2.1.0/ (ODbL, 100 patients), https://github.com/glee4810/FHIR-AgentBench (CC BY 4.0, 2,931 questions) and https://github.com/mooshifrew/fhirpath-qa (MIT). Targets: FHIR-AgentBench's best agent reached 50.0% correct answers and 0.68 retrieval recall. FHIRPath-QA's best zero-shot model reached 42%.
15. **Eunomia Synthea cancer sets in DuckDB.** https://github.com/OHDSI/EunomiaDatasets. Breast, lung and prostate sets of about 10,700 people each, in CDM 5.3. Target: the lung set gives exactly 10,756 persons and 374,965 records. No public synthetic data has the 5.4 Episode tables, so those rows would need synthesizing.
16. **Synthea cancer modules with ETL-Synthea.** https://github.com/synthetichealth/synthea (Apache-2.0) and https://github.com/OHDSI/ETL-Synthea. Target: our FHIR-to-OMOP output for one seeded population matches ETL-Synthea's tables, reported per table.
17. **Measured mapping loss.** TermX (Frontiers in Medicine 2026, https://pmc.ncbi.nlm.nih.gov/articles/PMC12935678/) reports 74% coverage from FHIR to OMOP and about 23% back. CareEvolution's OHDSI 2025 report found about 19% of conditions landing in OMOP's Observation table. MENDS-on-FHIR (JAMIA Open 2024) reports under 1% validation failure from OMOP to FHIR. Target: our loss reports use the same measures, and our numbers sit beside theirs.
18. **OMOP text-to-SQL benchmark.** https://github.com/red1dwarf/TEXT-TO-OMOP. 6,690 questions from 75 templates on CDM 5.4, with a synthetic database. Target: 93.55% execution accuracy, and correct abstention on the unanswerable items.
19. **Synthetic Hospital.** arXiv 2609.30027. 1,268 open synthetic patients coded in ICD-10-CM, SNOMED CT and LOINC. The best model rebuilds the problem list at a severity-weighted F1 of 0.73.
20. **The GENIE Data Model.** https://github.com/AACR-project-GENIE/GENIE-data-model, version 1.0.1, CC BY-NC-SA 4.0. A solid-tumor, person-centric model of 37 entities and 753 attributes, aligned with FHIR and OMOP, that keeps a source and a standard concept for each coded value and separates abstracted from derived values. Its license forbids commercial use and requires derivatives to carry the same license, so this repository copies none of its entities, attributes or value sets. Target: a private mapping of each element to BioData, FHIR and OMOP reports what BioData holds, and an adapter reads data in this shape into BioData values with its losses reported.
21. **FHIR Genomics Operations.** https://github.com/FHIR/genomics-operations, Apache-2.0, with synthetic data and a public demo server. Target: the same answers as the server for its operation examples.

## Cohort results to reproduce

22. **TCGA lung adenocarcinoma on cBioPortal.** Target: survival split by EGFR mutation (log-rank p = 0.125) and by EGFR amplification (p ≈ 0.0028).
23. **TCGA PanCancer pathways.** Target: 89% of 9,125 tumors carry a driver alteration in ten pathways (Sanchez-Vega, Cell 2018).
24. **MSK-IMPACT 50K.** https://www.cbioportal.org/study/summary?id=msk_impact_50k_2026. Target: 54,331 tumors from 48,179 patients across 448 subtypes (Cancer Cell 2026).
25. **Panels against whole genomes.** An 888-patient Nature Medicine study found actionable markers in 49% of patients with a simulated 50-gene panel and 70% with a 523-gene panel. Whole genomes added findings in 8%. Target: rerun the panel simulation on a public cohort.

## Needs Ian's approval

These need an account, a data use agreement or a license that limits use:

- MSK-CHORD, open on cBioPortal but licensed CC BY-NC-ND 4.0.
- MedAgentBench, real de-identified records with no stated data license.
- OHDSI Athena vocabularies, which need an account, with some vocabularies licensed separately.
- Credentialed sets: full MIMIC-IV, EHRSHOT, MedAlign, the original EHRSQL and n2c2 2018.

OncoKB-based results are left out because OncoKB is licensed.

## Problem shapes to keep as cases

- A page break in a registry record splits a criterion from its exception.
- An exact search for a rare KRAS variant misses broader RAS trials.
- Two variants on one allele must stay together.
- "Not documented" differs from "criterion not met".
- "No more than three prior lines" is a ceiling the trial value must store.
- Some trials sit only in hospital pages and non-US registries such as CTIS, ChiCTR and ANZCTR.
- A source that overwrites itself makes an analysis irreproducible, so each run records its data vintage.
