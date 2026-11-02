#!/usr/bin/env python3
"""Check that the reconciliation record has a row for every public item.

Usage: check-reconciliation.py BIOMCP_REPO SECOND_REPO [--list] [--record PATH]
       check-reconciliation.py --merged

Each REPO is a local Git clone of a consumer. The script reads each crate's
commit from the record, extracts `crates/biodata` at that commit into a
temporary folder with the consumer's workspace manifest, lock file and
toolchain file, and asks rustdoc for the crate's public items. rustdoc's
JSON output is unstable, so the script sets RUSTC_BOOTSTRAP for that one
build, and it includes items hidden from the docs. An item is a public struct, enum, trait, function, constant, static,
type alias, macro or module, including those a macro defines. Fields,
variants and methods follow the rule of the item that holds them.

Rows are the table lines under the record's Items heading, up to the next
`## ` heading. The first two cells name the item in each crate as `Name` in
`file`, or `none`. The third cell holds exactly one rule named in the
record's Rules table. The fifth cell states the merged public names and files.
The --merged mode compiles the current public API and checks every final location. A MOVE row names the item in one crate only; a ONE,
ONE-W or NAME row names it in both.

The check fails when a public item has no row, when a row names an item
the crate does not have, when an item has more than one row, when a row's
rule is missing or unknown or does not fit its cells, when the Rules
table defines any rule but MOVE, ONE, ONE-W and NAME, when two public
items share one name in one file, or when a public re-export resolves to
no item. Output is
a one-line summary per crate plus at most 40 problem lines; --list also
prints every item.
"""

import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

RECORD = Path(__file__).resolve().parents[1] / "sdlc" / "designs" / "2026-10-28-reconciliation.md"
CRATES = ("biomcp", "second")
COMMIT_LINE = {
    "biomcp": re.compile(r"^BioMCP commit: `([0-9a-f]{40})`", re.M),
    "second": re.compile(r"^Second consumer commit: `([0-9a-f]{40})`", re.M),
}
KINDS = {"struct", "enum", "trait", "function", "constant", "static",
         "type_alias", "macro", "union", "module"}
NONE = "none"
PROBLEM_LIMIT = 40
MOVE = "MOVE"
BOTH = {"ONE", "ONE-W", "NAME"}


def git(repo, *args):
    return subprocess.run(["git", "-C", repo, *args], check=True,
                          capture_output=True).stdout


def workspace_manifest(text):
    """Keep the consumer's workspace settings with only the biodata member."""
    return re.sub(r"(?m)^members\s*=\s*\[[^\]]*\]",
                  'members = ["crates/biodata"]', text, count=1)


def public_items(repo, commit, scratch):
    """Return {(name, file): kind} for the crate at commit."""
    root = Path(scratch)
    archive = git(repo, "archive", "--format=tar", commit, "crates/biodata")
    # System tar, because tarfile's safe extraction filter needs Python 3.12.
    subprocess.run(["tar", "-x", "-C", str(root)], input=archive, check=True)
    manifest = git(repo, "show", f"{commit}:Cargo.toml").decode()
    (root / "Cargo.toml").write_text(workspace_manifest(manifest))
    for name in ("Cargo.lock", "rust-toolchain.toml"):
        (root / name).write_bytes(git(repo, "show", f"{commit}:{name}"))
    env = dict(os.environ, RUSTC_BOOTSTRAP="1", CARGO_TARGET_DIR=str(root / "target"))
    built = subprocess.run(
        ["cargo", "rustdoc", "--quiet", "-p", "biodata",
         "--lib", "--all-features", "--", "-Zunstable-options", "--output-format", "json",
         "--document-hidden-items"],
        cwd=root, env=env, capture_output=True, text=True)
    if built.returncode != 0:
        raise SystemExit(f"rustdoc failed for {commit}:\n{built.stderr[-4000:]}")
    doc = json.loads((root / "target" / "doc" / "biodata.json").read_text())
    return walk(doc)


def walk(doc):
    index = doc["index"]
    found = {}
    owner = {}
    seen = set()

    def visit(item_id):
        item = index.get(str(item_id))
        if item is None or item["crate_id"] != 0 or item_id in seen:
            return
        seen.add(item_id)
        kind, inner = next(iter(item["inner"].items()))
        if kind == "use":
            if inner["id"] is None:
                raise SystemExit(f"re-export `{inner['source']}` resolves to no item")
            if inner["is_glob"]:
                visit_module(inner["id"])
            else:
                visit(inner["id"])
            return
        if kind == "module":
            if not inner["is_crate"]:
                record(item_id, item, kind)
            visit_module(item_id)
            return
        if kind in KINDS:
            record(item_id, item, kind)

    def visit_module(module_id):
        module = index.get(str(module_id))
        if module is None:
            return
        for child in module["inner"]["module"]["items"]:
            child_item = index.get(str(child))
            if child_item is not None and child_item["visibility"] == "public":
                visit(child)

    def record(item_id, item, kind):
        key = (item["name"], item["span"]["filename"].removeprefix("crates/biodata/"))
        if owner.setdefault(key, item_id) != item_id:
            raise SystemExit(f"two public items are `{key[0]}` in `{key[1]}`; rows cannot tell them apart")
        found[key] = kind

    visit(doc["root"])
    return found


CELL = re.compile(r"^`([^`]+)` in `([^`]+)`$")


def section(text, heading):
    """Return the lines under a `## ` heading, up to the next one."""
    _, found, rest = text.partition(f"\n## {heading}\n")
    if not found:
        raise SystemExit(f"record has no {heading} section")
    return re.split(r"(?m)^## ", rest, maxsplit=1)[0].splitlines()


def cells(line):
    return [cell.strip() for cell in line.strip().strip("|").split("|")]


def rule_names(text):
    """Return the rule names that the record's Rules table defines."""
    names = {cells(line)[0] for line in section(text, "Rules")
             if line.startswith("| ") and not line.startswith(("| Rule ", "|--"))}
    if names != BOTH | {MOVE}:
        raise SystemExit(f"Rules table must define exactly {sorted(BOTH | {MOVE})}, "
                         f"not {sorted(names)}")
    return names


def record_rows(text):
    """Return per-crate lists of (name, file) named by the rows under Items."""
    rules = rule_names(text)
    rows = {crate: [] for crate in CRATES}
    for line in section(text, "Items"):
        if not line.startswith("|") or line.startswith(("| BioMCP ", "|--")):
            continue
        row = cells(line)
        if len(row) != 5:
            raise SystemExit(f"row does not have five cells: {line!r}")
        named = [cell != NONE for cell in row[:2]]
        rule = row[2]
        if rule not in rules:
            raise SystemExit(f"row rule is not one rule from the Rules table: {line!r}")
        if (rule == MOVE and sum(named) != 1) or (rule in BOTH and not all(named)) or not any(named):
            raise SystemExit(f"row rule {rule} does not fit its cells: {line!r}")
        for crate, cell in zip(CRATES, row[:2]):
            if cell == NONE:
                continue
            for part in cell.split("<br>"):
                match = CELL.match(part.strip())
                if match is None:
                    raise SystemExit(f"row cell is not `Name` in `file`: {part!r}")
                rows[crate].append((match.group(1), match.group(2)))
    for crate, items in rows.items():
        if len(items) != len(set(items)):
            raise SystemExit(f"{crate} public item has more than one row")
    return rows


RENAMED = {
    ("biomcp", "GeneRef"): "SourceGeneRef",
    ("second", "Error"): "SourceShapeError",
    ("biomcp", "ClinvarReviewStatusCode"): "ReviewStatus",
    ("biomcp", "ClinvarAlleleOriginCode"): "AlleleOrigin",
}


def merged_items():
    """Read the public API that this checkout actually compiles."""
    env = dict(os.environ, RUSTC_BOOTSTRAP="1")
    subprocess.run(["cargo", "rustdoc", "--quiet", "-p", "biodata", "--lib",
                    "--all-features", "--locked", "--", "-Zunstable-options",
                    "--output-format", "json", "--document-hidden-items"],
                   env=env, check=True)
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"]))
    return walk(json.loads((Path(metadata["target_directory"]) / "doc" / "biodata.json").read_text()))


def check_merged(text, actual):
    """Bind every source item to the recorded merged name and file."""
    record_rows(text)
    covered = set()
    problems = []
    for line in section(text, "Items"):
        if not line.startswith("| ") or line.startswith("| BioMCP "):
            continue
        row = cells(line)
        targets = set()
        for part in row[4].split("<br>"):
            match = CELL.fullmatch(part.strip())
            if match is None:
                problems.append("merged cell is not `Name` in `file`")
                continue
            target = match.groups()
            targets.add(target)
            if target not in actual:
                problems.append(f"merged item absent: `{target[0]}` in `{target[1]}`")
        expected = set()
        for crate, cell in zip(CRATES, row[:2]):
            if cell == NONE:
                continue
            for part in cell.split("<br>"):
                name, _ = CELL.fullmatch(part.strip()).groups()
                expected.add(RENAMED.get((crate, name), name))
        if expected != {name for name, _ in targets}:
            problems.append("merged names do not implement the row's source items")
        for name, path in sorted(covered & targets):
            problems.append(f"merged public item has more than one row: `{name}` in `{path}`")
        covered.update(targets)
    if "\n## Approved API additions\n" in text:
        for line in section(text, "Approved API additions"):
            if not line.startswith("| ") or line.startswith("| Public item "):
                continue
            match = CELL.fullmatch(cells(line)[0])
            if match is None:
                problems.append("added API is not `Name` in `file`")
                continue
            target = match.groups()
            if target not in actual:
                problems.append(f"added public API absent: `{target[0]}` in `{target[1]}`")
            if target in covered:
                problems.append(f"added public API already has a source row: `{target[0]}`")
            covered.add(target)
    for name, path in sorted(set(actual) - covered):
        problems.append(f"merged public item has no row: `{name}` in `{path}`")
    print(f"merged: {len(actual)} public items, {len(covered)} recorded locations, {len(problems)} problems")
    for problem in problems[:PROBLEM_LIMIT]:
        print(f"  {problem}")
    return bool(problems)


def main(argv):
    args = list(argv)
    if args == ["--merged"]:
        return int(check_merged(RECORD.read_text(), merged_items()))
    record_path = RECORD
    if "--record" in args:
        at = args.index("--record")
        if at + 1 == len(args):
            raise SystemExit(__doc__)
        record_path = Path(args.pop(at + 1))
        args.pop(at)
    args = [arg for arg in args if arg != "--list"]
    if len(args) != 2 or any(arg.startswith("--") for arg in args):
        raise SystemExit(__doc__)
    text = record_path.read_text()
    rows = record_rows(text)
    failed = False
    for crate, repo in zip(CRATES, args):
        match = COMMIT_LINE[crate].search(text)
        if match is None:
            raise SystemExit(f"record states no {crate} commit")
        commit = match.group(1)
        with tempfile.TemporaryDirectory() as scratch:
            items = public_items(repo, commit, scratch)
        named = rows[crate]
        problems = [f"no row: `{n}` in `{f}` ({items[(n, f)]})"
                    for n, f in sorted(set(items) - set(named))]
        problems += [f"row names no public item: `{n}` in `{f}`"
                     for n, f in sorted(set(named) - set(items))]
        problems += [f"more than one row: `{n}` in `{f}`"
                     for n, f in sorted({key for key in named if named.count(key) > 1})]
        print(f"{crate} {commit[:12]}: {len(items)} public items, "
              f"{len(set(named))} named by rows, {len(problems)} problems")
        if "--list" in argv:
            for (name, path), kind in sorted(items.items(), key=lambda kv: (kv[0][1], kv[0][0])):
                print(f"  {kind}\t{name}\t{path}")
        for problem in problems[:PROBLEM_LIMIT]:
            print(f"  {problem}")
        if len(problems) > PROBLEM_LIMIT:
            print(f"  ... {len(problems) - PROBLEM_LIMIT} more")
        failed = failed or bool(problems)
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
