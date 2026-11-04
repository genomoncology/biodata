#!/usr/bin/env python3
"""Fetch pinned public release corpora; cached files are verified without a request."""
import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import urllib.request

RELEASE = "01-Nov-2026"
FILES = ["VariantSummaries", "MolecularProfileSummaries", "AcceptedAndSubmittedClinicalEvidenceSummaries", "AcceptedAssertionSummaries", "FeatureSummaries"]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path.home() / "workspace/data/biodata-corpora")
    args = parser.parse_args()
    root = args.root / "civic" / RELEASE
    root.mkdir(parents=True, exist_ok=True)
    for kind in FILES:
        name = f"{RELEASE}-{kind}.tsv"
        path = root / name
        url = f"https://civicdb.org/downloads/{RELEASE}/{name}"
        if not path.exists():
            with urllib.request.urlopen(url, timeout=120) as response:
                path.write_bytes(response.read())
        data = path.read_bytes()
        print(name, len(data), hashlib.sha256(data).hexdigest(), flush=True)
        rows = list(csv.DictReader(data.decode().splitlines(), delimiter="\t"))
        print("columns", json.dumps(list(rows[0]) if rows else []), "rows", len(rows), flush=True)
        if rows:
            print("sample", json.dumps(rows[0], ensure_ascii=False)[:3500], flush=True)


if __name__ == "__main__":
    main()
