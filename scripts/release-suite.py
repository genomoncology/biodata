#!/usr/bin/env python3
"""Run offline public release checks. Prints counts; never fetches inputs."""
import csv
from collections import Counter
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

from corpora import REPO, sources, verified_files


def main():
    try:
        rows = {}
        for source in sources():
            for entry, path in verified_files(source):
                with path.open(newline="") as stream:
                    records = list(csv.DictReader(stream, delimiter="\t"))
                if not records or any(None in row for row in records):
                    raise ValueError(f"empty or malformed TSV: {path.name}")
                rows[entry["kind"]] = records
        counts = Counter()
        for row in rows["variants"]:
            counts.update({row[key] for key in ("gene", "5_prime_partner", "3_prime_partner") if row[key]})
        rows["gene_counts"] = dict(counts)
        with tempfile.TemporaryDirectory(prefix="biodata-release-") as folder:
            projection = Path(folder) / "civic.json"
            projection.write_text(json.dumps(rows))
            environment = dict(os.environ, BIODATA_RELEASE_ROWS=str(projection))
            return subprocess.run([
                "cargo", "test", "-p", "biodata", "--all-features", "--locked",
                "--test", "release_corpora", "--", "--ignored", "--nocapture",
            ], cwd=REPO, env=environment, check=False).returncode
    except (OSError, ValueError) as error:
        print(f"release suite: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
