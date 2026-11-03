#!/usr/bin/env python3
"""Prove make gate rejects an unused dependency, then passes after restoration.

Run in a clean checkout through pm job. Prints both gate runs and a final result.
Requires the gate's Rust toolchain and Python 3.11+.
"""
import os
from pathlib import Path
import subprocess
import tempfile


ROOT = Path(__file__).resolve().parents[1]


def main():
    os.chdir(ROOT)
    if subprocess.check_output(["git", "status", "--porcelain"]):
        raise SystemExit("dependency gate proof requires a clean checkout")
    manifest = ROOT / "crates/biodata/Cargo.toml"
    lock = ROOT / "Cargo.lock"
    original_manifest, original_lock = manifest.read_bytes(), lock.read_bytes()
    try:
        manifest.write_text(manifest.read_text().replace(
            "[dependencies]\n", '[dependencies]\nmemchr = "=2.8.3"\n', 1))
        subprocess.run(["cargo", "metadata", "--offline", "--format-version", "1"],
                       stdout=subprocess.DEVNULL, check=True)
        with tempfile.TemporaryFile(mode="w+") as output:
            result = subprocess.run(["make", "gate"], stdout=output,
                                    stderr=subprocess.STDOUT)
            output.seek(0)
            diagnostic = output.read()
        print(diagnostic, flush=True)
        if (result.returncode == 0 or "memchr" not in diagnostic
                or "biodata" not in diagnostic
                or "shear/unused_dependency" not in diagnostic):
            raise SystemExit("gate did not reject the injected unused dependency")
    finally:
        manifest.write_bytes(original_manifest)
        lock.write_bytes(original_lock)
    subprocess.run(["make", "gate"], check=True)
    if subprocess.check_output(["git", "status", "--porcelain"]):
        raise SystemExit("dependency gate proof left the checkout dirty")
    print("dependency gate: rejected unused memchr, passed after restoration; checkout clean")


if __name__ == "__main__":
    main()
