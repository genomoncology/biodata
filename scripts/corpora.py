"""Pinned public release inputs shared by fetch and offline checks."""
import hashlib
import os
from pathlib import Path
import tomllib

REPO = Path(__file__).resolve().parents[1]
MANIFEST = REPO / "scripts/corpora.toml"


def corpus_root():
    return Path(os.environ.get("BIODATA_CORPUS_ROOT", Path.home() / "workspace/data/biodata-corpora"))


def sources():
    with MANIFEST.open("rb") as source:
        return tomllib.load(source)["source"]


def digest(path):
    with path.open("rb") as source:
        return hashlib.file_digest(source, "sha256").hexdigest()


def verified_files(source):
    root = corpus_root() / source["name"] / source["release"]
    for entry in source["file"]:
        path = root / entry["name"]
        if not path.is_file():
            raise ValueError(f"missing corpus: {path}; run make fetch-corpora")
        if digest(path) != entry["sha256"]:
            raise ValueError(f"checksum mismatch: {path}")
        yield entry, path
