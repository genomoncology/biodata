//! Preserve source-native identities and check every CIViC release entity and notation.
use super::review::Review;
use biodata::{
    Concept, Curie, Doi, HgvsText, Outcome, Pmid, SourceGeneRef, Text, VariantRef, hgvs,
};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::str::FromStr;

type Row = BTreeMap<String, String>;
type Check<T> = Result<T, String>;

#[derive(Deserialize)]
struct Release {
    variants: Vec<Row>,
    profiles: Vec<Row>,
    evidence: Vec<Row>,
    assertions: Vec<Row>,
    features: Vec<Row>,
    gene_counts: BTreeMap<String, usize>,
}

#[derive(Default)]
struct Counts {
    parsed: usize,
    refused: usize,
    reviewed: usize,
}
impl Counts {
    fn add(&mut self, refused: bool, reviewed: bool) {
        self.refused += usize::from(refused);
        self.parsed += usize::from(!refused);
        self.reviewed += usize::from(reviewed);
    }
    #[expect(
        clippy::print_stdout,
        reason = "release suite reports one summary per source file"
    )]
    fn print(&self, file: &str) {
        println!(
            "{file}: parsed={} refused={} reviewed={}",
            self.parsed, self.refused, self.reviewed
        );
    }
}

fn field<'a>(row: &'a Row, key: &str) -> Check<&'a str> {
    row.get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("missing column {key}"))
}
fn parse<T: FromStr>(input: &str) -> Check<T>
where
    T::Err: std::fmt::Display,
{
    input.parse().map_err(|e| format!("invalid:{e}"))
}
fn text(input: &str) -> Check<Text> {
    parse(input)
}
fn status<T>(result: &Check<T>) -> &str {
    match result {
        Ok(_) => "parsed",
        Err(error) => error,
    }
}
fn list(input: &str) -> impl Iterator<Item = &str> {
    input
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
}
fn native(row: &Row) -> Check<VariantRef> {
    VariantRef::source_with_feature(
        parse::<Curie>(&format!("civic:{}", field(row, "variant_id")?))?,
        text(field(row, "variant")?)?,
        Some(parse::<Curie>(&format!(
            "civicfeature:{}",
            field(row, "feature_id")?
        ))?),
        Box::default(),
    )
    .map_err(|e| format!("invalid:{e}"))
}
fn notation(input: &str) -> Check<HgvsText> {
    let value = parse(input)?;
    match hgvs::parse(input).outcome {
        Outcome::Parsed(_) => Ok(value),
        Outcome::Invalid(r) => Err(format!("invalid:{}", r.code.name())),
        Outcome::Unsupported(r) => Err(format!("unsupported:{}", r.code.name())),
        Outcome::LimitExceeded(r) => Err(format!("limit_exceeded:{}", r.code.name())),
    }
}
fn genes(row: &Row) -> Check<Vec<SourceGeneRef>> {
    let mut labels = BTreeSet::new();
    for key in ["gene", "5_prime_partner", "3_prime_partner"] {
        let value = field(row, key)?;
        if !value.is_empty() {
            labels.insert(value);
        }
    }
    labels
        .into_iter()
        .map(|label| {
            SourceGeneRef::new(text(label)?, None, None, None).map_err(|e| format!("invalid:{e}"))
        })
        .collect()
}

fn variants(
    rows: &[Row],
    review: &mut Review,
) -> Result<(BTreeMap<String, VariantRef>, BTreeMap<String, usize>), Box<dyn Error>> {
    let mut values = BTreeMap::new();
    let mut gene_counts = BTreeMap::<String, usize>::new();
    let mut counts = Counts::default();
    for (index, row) in rows.iter().enumerate() {
        let number = index + 2;
        let id = field(row, "variant_id")?;
        let result = native(row);
        let mut refused = result.is_err();
        let mut reviewed = review.check(
            number,
            id,
            "variant",
            field(row, "variant")?,
            status(&result),
        )?;
        if let Ok(value) = result {
            values.insert(id.to_owned(), value);
        }
        let parsed_genes = genes(row)?;
        for gene in parsed_genes {
            *gene_counts
                .entry(gene.label().as_str().to_owned())
                .or_default() += 1;
        }
        for input in list(field(row, "hgvs_descriptions")?) {
            let result = notation(input);
            refused |= result.is_err();
            reviewed |= review.check(number, id, "hgvs", input, status(&result))?;
        }
        counts.add(refused, reviewed);
    }
    counts.print("VariantSummaries.tsv");
    Ok((values, gene_counts))
}

fn profile(row: &Row, variants: &BTreeMap<String, VariantRef>) -> Check<VariantRef> {
    let members = list(field(row, "variant_ids")?)
        .map(|id| {
            variants
                .get(id)
                .cloned()
                .ok_or_else(|| format!("unsupported:civic.profile.member_missing:{id}"))
        })
        .collect::<Check<Vec<_>>>()?;
    VariantRef::profile(
        text(field(row, "molecular_profile_id")?)?,
        text(field(row, "name")?)?,
        members.into(),
    )
    .map_err(|e| format!("invalid:{e}"))
}
fn profiles(
    rows: &[Row],
    values: &BTreeMap<String, VariantRef>,
    review: &mut Review,
) -> Result<(), Box<dyn Error>> {
    let mut counts = Counts::default();
    for (index, row) in rows.iter().enumerate() {
        let result = profile(row, values);
        let reviewed = review.check(
            index + 2,
            field(row, "molecular_profile_id")?,
            "profile",
            field(row, "name")?,
            status(&result),
        )?;
        counts.add(result.is_err(), reviewed);
    }
    counts.print("MolecularProfileSummaries.tsv");
    Ok(())
}

fn concept(label: &str, id: Option<&str>) -> Check<Concept> {
    Ok(Concept {
        id: id.map(parse).transpose()?,
        label: text(label)?,
    })
}
fn citation(row: &Row) -> Check<()> {
    text(field(row, "citation")?)?;
    let id = field(row, "citation_id")?;
    match field(row, "source_type")? {
        "PubMed" => {
            parse::<Pmid>(id)?;
        }
        "ASCO" | "ASH" => {
            parse::<Doi>(id)?;
        }
        other => return Err(format!("unsupported:civic.citation.source:{other}")),
    }
    Ok(())
}
fn clinical(rows: &[Row], kind: &str, review: &mut Review) -> Result<(), Box<dyn Error>> {
    let mut counts = Counts::default();
    for (index, row) in rows.iter().enumerate() {
        let id = field(
            row,
            if kind == "evidence" {
                "evidence_id"
            } else {
                "assertion_id"
            },
        )?;
        let mut refused = false;
        let mut reviewed = false;
        if !field(row, "disease")?.is_empty() {
            let doid = field(row, "doid")?;
            let id = (!doid.is_empty()).then(|| format!("DOID:{doid}"));
            let result = concept(field(row, "disease")?, id.as_deref());
            refused |= result.is_err();
            reviewed |= review.check(
                index + 2,
                field(
                    row,
                    if kind == "evidence" {
                        "evidence_id"
                    } else {
                        "assertion_id"
                    },
                )?,
                &format!("{kind}.disease"),
                field(row, "disease")?,
                status(&result),
            )?;
        }
        for input in list(field(row, "therapies")?) {
            let result = concept(input, None);
            refused |= result.is_err();
            reviewed |= review.check(
                index + 2,
                id,
                &format!("{kind}.therapy"),
                input,
                status(&result),
            )?;
        }
        if kind == "evidence" {
            let result = citation(row);
            refused |= result.is_err();
            reviewed |= review.check(
                index + 2,
                id,
                "citation",
                field(row, "citation_id")?,
                status(&result),
            )?;
        }
        counts.add(refused, reviewed);
    }
    counts.print(if kind == "evidence" {
        "AcceptedAndSubmittedClinicalEvidenceSummaries.tsv"
    } else {
        "AcceptedAssertionSummaries.tsv"
    });
    Ok(())
}

pub(super) fn run() -> Result<(), Box<dyn Error>> {
    let path = std::env::var("BIODATA_RELEASE_ROWS")?;
    let release: Release = serde_json::from_reader(std::fs::File::open(path)?)?;
    let mut review = Review::load()?;
    let (values, counts) = variants(&release.variants, &mut review)?;
    assert_eq!(
        counts, release.gene_counts,
        "per-gene counts must match source TSV"
    );
    print_genes(&counts);
    profiles(&release.profiles, &values, &mut review)?;
    clinical(&release.evidence, "evidence", &mut review)?;
    clinical(&release.assertions, "assertions", &mut review)?;
    let mut counts = Counts::default();
    for (index, row) in release.features.iter().enumerate() {
        let result = text(field(row, "name")?);
        let reviewed = review.check(
            index + 2,
            field(row, "feature_id")?,
            "feature",
            field(row, "name")?,
            status(&result),
        )?;
        counts.add(result.is_err(), reviewed);
    }
    counts.print("FeatureSummaries.tsv");
    review.finish()
}
#[expect(
    clippy::print_stdout,
    reason = "release suite explicitly reports per-gene variant counts"
)]
fn print_genes(counts: &BTreeMap<String, usize>) {
    for (gene, count) in counts {
        println!("gene\t{gene}\t{count}");
    }
}
