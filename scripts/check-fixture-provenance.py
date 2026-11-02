#!/usr/bin/env python3
"""Check test data against local provenance.toml records (Python 3.11+).

Usage: check-fixture-provenance.py [TEST_ROOT]
Defaults to crates/biodata/tests in this checkout. Prints one summary on
success or one diagnostic on failure. Does not access the network.
"""

import argparse
import hashlib
import sys
try:
    import tomllib
except ModuleNotFoundError:
    sys.exit("fixture provenance requires Python 3.11+; set make PYTHON to a supported interpreter")
from pathlib import Path

ALLOWED_LICENSES = {
    "synthetic", "CC0-1.0", "LicenseRef-public-domain", "MIT", "Apache-2.0",
    "BSD-2-Clause", "BSD-3-Clause",
}
DEFAULT_ROOT = Path(__file__).resolve().parents[1] / "crates/biodata/tests"


def local_path(record, value):
    """Require a record-relative path without symlink or parent escapes."""
    if not isinstance(value, str) or not value.strip():
        raise ValueError(f"{record}: missing file path")
    path = Path(value)
    if path.is_absolute() or ".." in path.parts:
        raise ValueError(f"{record}: path outside record folder: {value}")
    result = record.parent / path
    if not result.resolve().is_relative_to(record.parent.resolve()):
        raise ValueError(f"{record}: path outside record folder: {value}")
    return result


def check(root):
    """Validate records, then require a record for every data file."""
    listed = set()
    licenses = set()
    for record in sorted(root.rglob("provenance.toml")):
        if record.is_symlink():
            raise ValueError(f"symlink is not provenance: {record.relative_to(root)}")
        with record.open("rb") as source:
            metadata = tomllib.load(source)
        license_name = metadata.get("license")
        if license_name not in ALLOWED_LICENSES:
            raise ValueError(f"{record.relative_to(root)}: unapproved license: {license_name}")
        if license_name != "synthetic" and not metadata.get("origin", "").strip():
            raise ValueError(f"{record.relative_to(root)}: missing origin")
        if "license_file" in metadata:
            notice = local_path(record, metadata["license_file"])
            if not notice.is_file():
                raise ValueError(f"missing license file: {notice.relative_to(root)}")
            licenses.add(notice)
        entries = metadata.get("file", [])
        if not isinstance(entries, list):
            raise ValueError(f"{record.relative_to(root)}: file must be an array of tables")
        for entry in entries:
            if not isinstance(entry, dict):
                raise ValueError(f"{record.relative_to(root)}: invalid file record")
            path = local_path(record, entry.get("path"))
            name = path.relative_to(root)
            if path in listed:
                raise ValueError(f"duplicate provenance: {name}")
            if not path.is_file():
                raise ValueError(f"missing file: {name}")
            with path.open("rb") as source:
                digest = hashlib.file_digest(source, "sha256").hexdigest()
            if digest != entry.get("sha256"):
                raise ValueError(f"hash mismatch: {name}")
            listed.add(path)
    count = 0
    for path in sorted(root.rglob("*")):
        if path.is_symlink():
            raise ValueError(f"symlink is not fixture data: {path.relative_to(root)}")
        if not path.is_file() or path.suffix == ".rs" or path.name == "provenance.toml":
            continue
        if path in licenses:
            continue
        if path not in listed:
            raise ValueError(f"no provenance: {path.relative_to(root)}")
        count += 1
    return count


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("test_root", nargs="?", type=Path, default=DEFAULT_ROOT)
    args = parser.parse_args()
    try:
        if args.test_root.is_symlink():
            raise ValueError("test root cannot be a symlink")
        if (args.test_root != DEFAULT_ROOT or args.test_root.exists()) and not args.test_root.is_dir():
            raise ValueError("test root is not a directory")
        count = check(args.test_root)
    except (OSError, ValueError, TypeError, AttributeError) as error:
        print(f"fixture provenance: {error}", file=sys.stderr)
        return 1
    print(f"fixture provenance: {count} data files checked")
    return 0


if __name__ == "__main__":
    sys.exit(main())
