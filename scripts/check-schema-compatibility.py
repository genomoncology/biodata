#!/usr/bin/env python3
"""Compare every public schema and identity with a Git revision; summarize changed paths."""
import argparse
import copy
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def run(args, root, env, **kwargs):
    return subprocess.run(args, cwd=root, env=env, check=True, **kwargs)


def public_schema_types(doc):
    index = doc["index"]
    paths = {}

    def visit(ident, prefix, alias=None, parents=()):
        item = index.get(str(ident))
        if item is None or item["crate_id"] != 0 or ident in parents:
            return
        kind, inner = next(iter(item["inner"].items()))
        if kind == "use":
            if inner["is_glob"]:
                module = index[str(inner["id"])]["inner"]["module"]
                for child in module["items"]:
                    if index[str(child)]["visibility"] == "public":
                        visit(child, prefix, parents=parents + (ident,))
            else:
                visit(inner["id"], prefix, item["name"], parents + (ident,))
        elif kind == "module":
            route = prefix if inner["is_crate"] else prefix + (item["name"],)
            for child in inner["items"]:
                if index[str(child)]["visibility"] == "public":
                    visit(child, route, parents=parents + (ident,))
        elif kind in {"struct", "enum", "type_alias"}:
            name = "::".join(prefix + (alias or item["name"],))
            paths.setdefault(str(ident), set()).add(name)

    visit(doc["root"], ())
    schemas = set()
    generic_schemas = set()
    for item in index.values():
        impl = item["inner"].get("impl")
        if not impl or not impl.get("trait") or impl["trait"]["path"].split("::")[-1] != "JsonSchema":
            continue
        target = impl["for"].get("resolved_path")
        if target and str(target["id"]) in paths:
            if impl["generics"]["params"]:
                generic_schemas.add(str(target["id"]))
            schemas.add(str(target["id"]))
    # Compare the concrete forms already used by public fields and aliases.
    representatives = {
        "Identifier": ["Identifier<i64>", "Identifier<std::num::NonZeroU64>"],
        "Protein": ["Protein<biodata::UniProtAcc>", "Protein<biodata::UniProtIsoform>"],
        "WithUnit": ["WithUnit<biodata::FiniteNumber>", "WithUnit<biodata::FiniteNumber,biodata::FaersAgeUnit>"],
    }
    names = []
    for ident in schemas:
        name = min(paths[ident], key=lambda value: (value.count("::"), value))
        if ident in generic_schemas:
            if name not in representatives:
                raise SystemExit("A public generic schema needs an explicit concrete comparison: " + name)
            names.extend(representatives[name])
        else:
            names.append(name)
    return sorted(names)


def export(root, target):
    env = {**os.environ, "RUSTC_BOOTSTRAP": "1", "CARGO_TARGET_DIR": str(target)}
    run(["cargo", "rustdoc", "--quiet", "-p", "biodata", "--lib", "--all-features", "--locked", "--",
         "-Zunstable-options", "--output-format", "json", "--document-hidden-items"], root, env)
    names = public_schema_types(json.loads((target / "doc/biodata.json").read_text()))
    example = root / "crates/biodata/examples/schema_comparison.rs"
    example.parent.mkdir(exist_ok=True)
    values = []
    for name in names:
        values.append(json.dumps(name) + ':{"schema":schemars::schema_for!(biodata::' + name + ').to_value(),"schema_id":<biodata::' + name + ' as schemars::JsonSchema>::schema_id()}')
    example.write_text('//! Export public schema contracts.\n#![recursion_limit="2048"]\nfn main(){let schemas=schemars::json_schema!({' + ",".join(values) + '});println!("{}",schemas.to_value());}\n')
    try:
        result = run(["cargo", "run", "--quiet", "-p", "biodata", "--features", "schema", "--locked", "--example", "schema_comparison"], root, env, capture_output=True, text=True)
        return json.loads(result.stdout)
    except subprocess.CalledProcessError as error:
        raise SystemExit(error.stderr[-6000:]) from error
    finally:
        example.unlink()
        example.parent.rmdir()


# Only the two approved choices and their existing embedding locations may move.
CHOICE_ROOTS = {
    "Category": {"Category"}, "RegionKind": {"RegionKind"},
    "Region": {"RegionKind"},
    **{name: {"Category", "RegionKind"} for name in
       ["PopulationFrequency", "PredictorScore", "Statement", "Subject", "VariantRef"]},
}


def string_schema(value, root, seen=()):
    if not isinstance(value, dict):
        return False
    ref = value.get("$ref")
    if isinstance(ref, str) and ref.startswith("#/") and ref not in seen:
        target = root
        for key in ref[2:].split("/"):
            target = target[key.replace("~1", "/").replace("~0", "~")]
        if string_schema(target, root, seen + (ref,)):
            return True
    if value.get("type") in ("string", ["string"]) or isinstance(value.get("const"), str):
        return True
    if value.get("enum") and all(isinstance(item, str) for item in value["enum"]):
        return True
    return any(value.get(key) and all(string_schema(item, root, seen) for item in value[key])
               for key in ["anyOf", "oneOf"]) or any(
                   string_schema(item, root, seen) for item in value.get("allOf", []))


def string_choices(value, path="", root=None):
    root = value if root is None else root
    found = []
    if isinstance(value, dict):
        for key in ["anyOf", "oneOf"]:
            branches = value.get(key)
            if isinstance(branches, list) and branches and all(string_schema(branch, root) for branch in branches):
                found.append(path)
        for key, child in value.items():
            found += string_choices(child, path + "/" + key, root)
    elif isinstance(value, list):
        for index, child in enumerate(value):
            found += string_choices(child, path + "/" + str(index), root)
    return found


def resolve_schema(value, root, seen=()):
    if not isinstance(value, dict):
        return {}
    ref = value.get("$ref")
    if not isinstance(ref, str) or not ref.startswith("#/"):
        return value
    if ref in seen:
        return {}
    target = root
    for key in ref[2:].split("/"):
        target = target[key.replace("~1", "/").replace("~0", "~")]
    return resolve_schema(target, root, seen + (ref,))


def tagged_union(value, root, seen=()):
    value = resolve_schema(value, root)
    if id(value) in seen:
        return False
    seen += (id(value),)
    if any(tagged_union(part, root, seen) for part in value.get("allOf", [])):
        return True
    for key in ["anyOf", "oneOf"]:
        branches = value.get(key)
        if not isinstance(branches, list) or not branches:
            continue
        tags = []
        for branch in branches:
            branch = resolve_schema(branch, root)
            tags.append({name for name, prop in branch.get("properties", {}).items()
                         if name in branch.get("required", [])
                         and isinstance(resolve_schema(prop, root).get("const"), str)})
        if set.intersection(*tags):
            return True
    return False


def untagged_wrappers(value, root, path="/schema"):
    found = []
    if isinstance(value, dict):
        for key in ["anyOf", "oneOf"]:
            branches = value.get(key)
            if not isinstance(branches, list) or len(branches) < 2 or tagged_union(value, root):
                continue
            nullable = len(branches) == 2 and any(resolve_schema(branch, root).get("type") == "null" for branch in branches)
            if not nullable and any(tagged_union(branch, root) for branch in branches):
                found.append(path)
        for key, child in value.items():
            found += untagged_wrappers(child, root, path + "/" + key)
    elif isinstance(value, list):
        for index, child in enumerate(value):
            found += untagged_wrappers(child, root, path + "/" + str(index))
    return found


def check_choices(schemas):
    found = [(name, path) for name, value in schemas.items() for path in string_choices(value["schema"], "/schema")]
    found += [(name, path) for name, value in schemas.items()
              for path in untagged_wrappers(value["schema"], value["schema"])]
    if found:
        raise SystemExit("malformed union schemas: " + str(found))
    print(f"closed choices: {len(schemas)} exported roots checked", flush=True)


def approved_choice(name, path, before, after):
    choice = name if path == "/schema" else path.removeprefix("/schema/$defs/")
    if choice not in CHOICE_ROOTS.get(name, set()) or path not in {"/schema", "/schema/$defs/" + choice}:
        return False
    if not isinstance(before, dict) or not isinstance(before.get("oneOf"), list):
        return False
    branches = before["oneOf"]
    if not branches or not all(isinstance(branch, dict) and set(branch) == {"type", "const", "description"}
                               and branch["type"] == "string" and isinstance(branch["const"], str)
                               and isinstance(branch["description"], str) for branch in branches):
        return False
    expected = {key: value for key, value in before.items() if key != "oneOf"}
    expected.update(type="string", enum=[branch["const"] for branch in branches],
                    **{"x-enum-descriptions": [branch["description"] for branch in branches]})
    return expected == after


TRANSCRIPT_ROOTS = {
    "TranscriptReference": ["TranscriptReference"], "TranscriptId": ["TranscriptId"],
    "PredictorScore": ["TranscriptReference", "TranscriptId"],
    **{name: ["TranscriptReference"] for name in
       ["ProteinSite", "PopulationFrequency", "Statement", "Subject", "VariantRef"]},
}


def transcript_rewrite(name, value):
    expected = copy.deepcopy(value)
    schema = expected["schema"]
    for choice in TRANSCRIPT_ROOTS.get(name, []):
        node = schema if name == choice else schema["$defs"][choice]
        branches = node.get("anyOf")
        if not branches or [branch.get("$ref") for branch in branches] != [
                "#/$defs/Accession", "#/$defs/AccessionBase"] or any(
                    set(branch) != {"$ref", "description"} for branch in branches):
            raise SystemExit("Unexpected transcript union baseline: " + name)
        patterns = [schema["$defs"][component]["pattern"] for component in ["Accession", "AccessionBase"]]
        del node["anyOf"]
        node.update(type="string", pattern="(" + "|".join(patterns) + ")",
                    description="A transcript reference in versioned Accession or unversioned AccessionBase form.")
    if name in TRANSCRIPT_ROOTS:
        for component in ["Accession", "AccessionBase"]:
            if '"#/$defs/' + component + '"' not in json.dumps(schema):
                schema["$defs"].pop(component, None)
        if not schema["$defs"]:
            del schema["$defs"]
    return expected


def subject_rewrite(name, value):
    expected = copy.deepcopy(value)
    if name not in {"Subject", "Statement"}:
        return expected
    schema = expected["schema"]
    node = schema if name == "Subject" else schema["$defs"]["Subject"]
    if node.get("anyOf") != [{"$ref": "#/$defs/" + kind} for kind in
                             ["VariantRef", "GeneSubjectWire", "RegionSubjectWire"]]:
        raise SystemExit("Unexpected Subject wrapper baseline: " + name)
    branches = copy.deepcopy(schema["$defs"]["VariantRef"]["oneOf"])
    for kind, level in [("GeneSubjectWire", "gene"), ("RegionSubjectWire", "region")]:
        branch = copy.deepcopy(schema["$defs"][kind])
        branch["properties"]["level"] = {"const": level, "type": "string"}
        branches.append(branch)
    del node["anyOf"]
    node["oneOf"] = branches
    return expected


def differences(before, after, path="", name=""):
    if before == after:
        return []
    if approved_choice(name, path, before, after):
        return [(path, before, after)]
    if isinstance(before, dict) and isinstance(after, dict):
        result = []
        for key in sorted(before.keys() | after.keys()):
            if key not in before or key not in after:
                result.append((path + "/" + key, before.get(key), after.get(key)))
            else:
                result += differences(before[key], after[key], path + "/" + key, name)
        return result
    return [(path, before, after)]


def approved(name, path, before, after):
    if approved_choice(name, path, before, after):
        return True
    root = name == "AuthorId" and path.startswith("/schema/")
    nested = "/$defs/AuthorId/" in path
    if not root and not nested:
        return False
    if path.endswith("/pattern") and isinstance(before, str):
        return after == before.replace("(semantic-scholar|semanticscholar):", "semantic-scholar:")
    if path.endswith("/description") and isinstance(before, str):
        return "accepts semanticscholar:ID" in before and after == before.replace(", accepts semanticscholar:ID", "")
    return False


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline")
    parser.add_argument("--choices-only", action="store_true")
    parser.add_argument("--input", type=Path, help="Validate a retained raw export without rebuilding")
    parser.add_argument("--gate", action="store_true")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if not args.choices_only and not args.baseline:
        parser.error("comparison requires --baseline")
    if args.input and not args.choices_only:
        parser.error("--input requires --choices-only")
    if args.gate and args.choices_only:
        parser.error("--gate and --choices-only cannot be combined")
    args.output.mkdir(parents=True, exist_ok=True)
    if args.gate:
        run(["make", "gate", "SCHEMA_EXPORT=" + str(args.output.resolve())], ROOT, os.environ)
        current = json.loads((args.output / "candidate.json").read_text())
    else:
        current = json.loads(args.input.read_text()) if args.input else export(ROOT, ROOT / "target")
    check_choices(current)
    (args.output / "candidate.json").write_text(json.dumps(current, sort_keys=True, indent=2) + "\n")
    if args.choices_only:
        return
    with tempfile.TemporaryDirectory(prefix="biodata-schema-baseline-") as folder:
        baseline_root = Path(folder)
        archive = subprocess.check_output(["git", "archive", "--format=tar", args.baseline], cwd=ROOT)
        run(["tar", "-x", "-C", folder], ROOT, os.environ, input=archive)
        baseline = export(baseline_root, baseline_root / "target")
    if baseline.keys() != current.keys():
        raise SystemExit("Public schema root set changed: " + str(sorted(baseline.keys() ^ current.keys())))
    deltas = {name: differences(baseline[name], current[name], name=name) for name in baseline}
    changed = {name: delta for name, delta in deltas.items() if delta}
    transcript_deltas = {name: differences(baseline[name], transcript_rewrite(name, baseline[name]), name=name) for name in baseline}
    subject_deltas = {name: differences(baseline[name], subject_rewrite(name, baseline[name]), name=name) for name in baseline}
    unexpected = [(name, path) for name, delta in changed.items() for path, before, after in delta if not approved(name, path, before, after) and (path, before, after) not in transcript_deltas[name] and (path, before, after) not in subject_deltas[name]]
    args.output.mkdir(parents=True, exist_ok=True)
    for name, value in [("baseline", baseline), ("candidate", current), ("delta", changed)]:
        (args.output / (name + ".json")).write_text(json.dumps(value, sort_keys=True, indent=2) + "\n")
    print(f"public schemas: {len(baseline)} roots, {len(baseline)-len(changed)} identical, {len(changed)} contain approved changes, {len(unexpected)} unexpected differences", flush=True)
    for name, delta in changed.items():
        print(name + ": " + ", ".join(path for path, _, _ in delta), flush=True)
    if unexpected:
        raise SystemExit(str(unexpected[:30]))


if __name__ == "__main__":
    main()
