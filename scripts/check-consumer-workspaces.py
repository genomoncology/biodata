#!/usr/bin/env python3
"""Check isolated consumer snapshots against this checkout; print bounded diagnostics."""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def invoke(command, cwd, env, log):
    with log.open("w") as output:
        result = subprocess.run(command, cwd=cwd, env=env, stdout=output, stderr=subprocess.STDOUT)
    if result.returncode:
        # Compiler output can include consumer source. Keep raw logs private and
        # expose only diagnostic codes and counts to the public job log.
        text = log.read_text(errors="replace")
        codes = sorted(set(re.findall(r"error\[(E[0-9]+)\]", text)))
        print(f"check failed: exit {result.returncode}, codes {codes}; diagnostics retained at {log}", flush=True)
    return result.returncode


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--consumer", action="append", nargs=3, metavar=("SOURCE", "PIN", "MODES"), required=True,
                        help="Git source, exact revision, comma-separated modes: default, none, or a feature name")
    parser.add_argument("--rename", action="append", nargs=4, default=[], metavar=("CONSUMER", "FILE", "OLD", "NEW"),
                        help="Apply one authorized identifier rename only in the numbered disposable snapshot")
    parser.add_argument("--replace", action="append", nargs=4, default=[], metavar=("CONSUMER", "FILE", "BEFORE", "AFTER"),
                        help="Apply one authorized exact text replacement only in the numbered disposable snapshot")
    parser.add_argument("--rename-count", action="append", nargs=5, default=[],
                        metavar=("CONSUMER", "FILE", "OLD", "NEW", "COUNT"),
                        help="Apply an exact authorized number of identifier renames")
    parser.add_argument("--replace-count", action="append", nargs=5, default=[],
                        metavar=("CONSUMER", "FILE", "BEFORE", "AFTER", "COUNT"),
                        help="Apply an exact authorized number of text substitutions")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    candidate = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    records = []
    failed = False
    with tempfile.TemporaryDirectory(prefix="biodata-consumer-check-") as temporary:
        for number, (source, pin, modes) in enumerate(args.consumer, 1):
            snapshot = Path(temporary) / str(number)
            snapshot.mkdir()
            repository = snapshot / "repository"
            env = {**os.environ, "CARGO_TARGET_DIR": str(snapshot / "target")}
            setup_log = args.output / f"consumer-{number}-setup.log"
            if invoke(["git", "clone", "--quiet", "--no-checkout", source, str(repository)], ROOT, env, setup_log):
                failed = True
                continue
            if invoke(["git", "checkout", "--quiet", "--detach", pin], repository, env, setup_log):
                failed = True
                continue
            exact = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repository, text=True).strip()
            if exact != pin:
                raise SystemExit("consumer revision must be an exact full commit")
            adaptations = []
            edits = [("identifier", *edit, 1) for edit in args.rename] + [("text", *edit, 1) for edit in args.replace]
            edits += [("identifier", *edit[:-1], int(edit[-1])) for edit in args.rename_count]
            edits += [("text", *edit[:-1], int(edit[-1])) for edit in args.replace_count]
            for kind, selected, filename, before, after, expected in edits:
                if int(selected) != number:
                    continue
                path = (repository / filename).resolve()
                if repository.resolve() not in path.parents:
                    raise SystemExit("adaptation path must stay in its disposable snapshot")
                original = path.read_text()
                if kind == "identifier":
                    text, count = re.subn(r"\b" + re.escape(before) + r"\b", after, original)
                else:
                    count = original.count(before)
                    text = original.replace(before, after)
                if expected < 1 or count != expected:
                    raise SystemExit(f"authorized adaptation expected {expected} matches; found {count}")
                path.write_text(text)
                adaptations.append({"kind": kind, "file": filename, "before": before, "after": after, "count": count})
                print(f"consumer {number}: authorized {kind} adaptation {len(adaptations)}, count {count}", flush=True)
            manifest = repository / "Cargo.toml"
            text = manifest.read_text()
            # Preserve the consumer's requested features; replace only Git identity
            # with the exact candidate checkout path in this disposable snapshot.
            def dependency(match):
                fields = re.sub(r'\b(?:git|rev|tag|branch|version)\s*=\s*"[^"]*"\s*,?\s*', "", match.group(1))
                return 'biodata = { path = ' + json.dumps(str(ROOT / "crates/biodata")) + ', ' + fields + '}'
            text, count = re.subn(r'^biodata\s*=\s*\{([^\n}]*)\}', dependency, text, flags=re.MULTILINE)
            if count != 1:
                raise SystemExit("expected one workspace BioData dependency")
            manifest.write_text(text)
            for mode in modes.split(","):
                command = ["cargo", "check", "--workspace", "--all-targets", "--keep-going"]
                if mode == "none":
                    command.append("--no-default-features")
                elif mode != "default":
                    command += ["--features", mode]
                log = args.output / f"consumer-{number}-{mode.replace('/', '-')}.log"
                code = invoke(command, repository, env, log)
                records.append({"consumer": number, "revision": pin, "candidate": candidate,
                                "command": command, "exit": code, "log": log.name, "adaptations": adaptations})
                failed |= bool(code)
                print(f"consumer {number}, mode {mode}: {'FAILED' if code else 'passed'}", flush=True)
    (args.output / "results.json").write_text(json.dumps(records, indent=2) + "\n")
    raise SystemExit(int(failed))


if __name__ == "__main__":
    main()
