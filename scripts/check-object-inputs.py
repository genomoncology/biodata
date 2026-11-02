#!/usr/bin/env python3
"""Run object boundary witnesses, optionally against a frozen source revision."""
import argparse
import os
from pathlib import Path
import subprocess
import tempfile

root = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--baseline")
args = parser.parse_args()
command = ["cargo", "test", "-p", "biodata", "--test", "object_inputs", "--features", "schema", "--locked"]
if not args.baseline:
    raise SystemExit(subprocess.call(command, cwd=root))
with tempfile.TemporaryDirectory(prefix="biodata-object-baseline-") as folder:
    baseline = Path(folder)
    archive = subprocess.check_output(["git", "archive", "--format=tar", args.baseline], cwd=root)
    subprocess.run(["tar", "-x", "-C", folder], input=archive, check=True)
    witness = Path("crates/biodata/tests/object_inputs.rs")
    (baseline / witness).write_bytes((root / witness).read_bytes())
    environment = {**os.environ, "CARGO_TARGET_DIR": str(root / "target/object-baseline")}
    raise SystemExit(subprocess.call(command, cwd=baseline, env=environment))
