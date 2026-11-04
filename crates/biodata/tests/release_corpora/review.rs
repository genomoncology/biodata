//! Match exact release observations to reviewed differences; stale reviews also fail.
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::io::Write;

type Key = (String, String, String, String);

pub struct Review {
    rows: BTreeMap<Key, String>,
    seen: BTreeSet<Key>,
    failures: Vec<String>,
    observations: Option<std::fs::File>,
}

impl Review {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let path = std::env::var("BIODATA_REVIEWED_ROWS").ok();
        let source = match path {
            Some(path) => std::fs::read_to_string(path)?,
            None => include_str!("reviewed-differences.tsv").to_owned(),
        };
        let mut rows = BTreeMap::new();
        for row in source.lines().skip(1) {
            let fields: Vec<_> = row.split('\t').collect();
            let [corpus, number, id, kind, input, result, reason, rule] = fields.as_slice() else {
                return Err("reviewed difference must have eight columns".into());
            };
            if *corpus != "civic-01-Nov-2026" || reason.is_empty() || !rule.starts_with("https://")
            {
                return Err(format!("invalid reviewed difference: {row}").into());
            }
            let key = (
                number.to_string(),
                id.to_string(),
                kind.to_string(),
                input.to_string(),
            );
            if rows.insert(key, result.to_string()).is_some() {
                return Err("duplicate reviewed difference".into());
            }
        }
        let observations = std::env::var_os("BIODATA_RELEASE_OBSERVATIONS")
            .map(std::fs::File::create)
            .transpose()?;
        Ok(Self {
            rows,
            seen: BTreeSet::new(),
            failures: Vec::new(),
            observations,
        })
    }

    pub fn check(
        &mut self,
        row: usize,
        id: &str,
        kind: &str,
        input: &str,
        result: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let key = (
            row.to_string(),
            id.to_owned(),
            kind.to_owned(),
            input.to_owned(),
        );
        if let Some(file) = &mut self.observations {
            // Only refusal observations are retained; stdout stays bounded.
            if result != "parsed" {
                writeln!(
                    file,
                    "civic-01-Nov-2026\t{row}\t{id}\t{kind}\t{input}\t{result}"
                )?;
            }
        }
        match self.rows.get(&key) {
            Some(expected) if expected == result && result != "parsed" => {
                self.seen.insert(key);
                Ok(true)
            }
            _ if result == "parsed" && !self.rows.contains_key(&key) => Ok(false),
            _ => {
                self.failures
                    .push(format!("row {row} {id} {kind} {input}: {result}"));
                Ok(false)
            }
        }
    }

    pub fn finish(self) -> Result<(), Box<dyn Error>> {
        let stale: Vec<_> = self
            .rows
            .keys()
            .filter(|key| !self.seen.contains(*key))
            .collect();
        if !self.failures.is_empty() || !stale.is_empty() {
            return Err(format!(
                "{} unreviewed or changed refusals: {}; {} stale reviews: {:?}",
                self.failures.len(),
                self.failures
                    .iter()
                    .take(10)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("; "),
                stale.len(),
                stale.iter().take(3).collect::<Vec<_>>()
            )
            .into());
        }
        Ok(())
    }
}
