#!/usr/bin/env python3
"""Check public macros in featureless caller crates; print one result per mode."""
import argparse
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "crates/biodata/tests/caller"


def check(mode, folder):
    features = {"basic": [], "serde": ["serde"], "schema": ["schema"]}[mode]
    crate = folder / mode
    (crate / "src").mkdir(parents=True)
    deps = f'values={{package="biodata",path="{ROOT / "crates/biodata"}",default-features=false,features={features!r}}}\n'
    deps = deps.replace("'", '"')
    if mode != "basic":
        deps += 'serde_json="1.0.154"\n'
    if mode == "schema":
        deps += 'schema={package="schemars",version="1.2.3"}\n'
    (crate / "Cargo.toml").write_text('[package]\nname="macro-caller"\nversion="0.1.0"\nedition="2024"\n[workspace]\n[dependencies]\n' + deps)
    source = (FIXTURES / "common.rs").read_text() + "\n" + (FIXTURES / f"{mode}.rs").read_text()
    (crate / "src/main.rs").write_text(source)
    subprocess.run(["cargo", "run", "--quiet", "--manifest-path", str(crate / "Cargo.toml")], check=True,
                   env={**os.environ, "CARGO_TARGET_DIR": str(crate / "target")})
    if mode == "basic":
        for literal in ['""', '"\\u{202e}unsafe"']:
            (crate / "src/main.rs").write_text("fn main() { let _ = values::text!(" + literal + "); }")
            result = subprocess.run(["cargo", "check", "--quiet", "--manifest-path", str(crate / "Cargo.toml")], capture_output=True, text=True,
                                    env={**os.environ, "CARGO_TARGET_DIR": str(crate / "target")})
            if result.returncode == 0 or "E0080" not in result.stderr:
                raise SystemExit("invalid Text literal must fail const evaluation: " + result.stderr[-2000:])
    print(f"macro caller {mode}: passed", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mode", choices=["basic", "serde", "schema"])
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix="biodata-macro-callers-") as folder:
        for mode in [args.mode] if args.mode else ["basic", "serde", "schema"]:
            check(mode, Path(folder))


if __name__ == "__main__":
    main()
