#!/usr/bin/env python3
"""Prove release corpus acceptance, a removed review failure, and cached fetching."""
from pathlib import Path
import os
import subprocess
import tempfile

from corpora import REPO, corpus_root


def run(*arguments, environment=None):
    result = subprocess.run(["make", *arguments], cwd=REPO, env=environment,
                            text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, check=False)
    return result


def prove(environment):
    first = run("fetch-corpora", environment=environment)
    if first.returncode:
        raise SystemExit(first.stdout)
    print(first.stdout, end="", flush=True)
    accepted = run("release-suite", environment=environment)
    if accepted.returncode:
        raise SystemExit(accepted.stdout)
    print(accepted.stdout, end="", flush=True)
    reviewed = REPO / "crates/biodata/tests/release_corpora/reviewed-differences.tsv"
    lines = reviewed.read_text().splitlines()
    if len(lines) < 2:
        raise SystemExit("proof needs a reviewed release refusal")
    removed = lines[1].split("\t")
    with tempfile.TemporaryDirectory(prefix="biodata-missing-review-") as folder:
        path = Path(folder) / "reviewed.tsv"
        path.write_text("\n".join([lines[0], *lines[2:]]) + "\n")
        rejected = run("release-suite", environment=dict(environment, BIODATA_REVIEWED_ROWS=str(path)))
        if not rejected.returncode or not all(value in rejected.stdout for value in removed[1:5]):
            raise SystemExit("suite did not name the removed review: " + rejected.stdout[-5000:])
    print(f"removed review refused: row {removed[1]} {removed[2]} {removed[3]} {removed[4]}", flush=True)
    cached = run("fetch-corpora", environment=environment)
    if cached.returncode or "0 downloaded" not in cached.stdout:
        raise SystemExit("cached fetch downloaded or failed: " + cached.stdout)
    print(cached.stdout, end="", flush=True)
    # Routine tests must not read a release data root, even when one is configured.
    gate = run("gate", environment=dict(environment, BIODATA_CORPUS_ROOT="/dev/null/not-a-corpus"))
    if gate.returncode:
        raise SystemExit(gate.stdout)
    print("routine gate passed with an unusable release data root", flush=True)


def main():
    # Prove both cold downloads and verified cache hits without disturbing retained inputs.
    root = corpus_root()
    root.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="fetch-proof-", dir=root) as folder:
        prove(dict(os.environ, BIODATA_CORPUS_ROOT=folder))


if __name__ == "__main__":
    main()
