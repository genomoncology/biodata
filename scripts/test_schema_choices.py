"""Exercise the retained-export schema guard through its public command."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("check-schema-compatibility.py")


class SchemaChoices(unittest.TestCase):
    def assert_union(self, key, branches, definitions, refused):
        with self.subTest(key=key, branches=branches), tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            source = root / "schemas.json"
            source.write_text(json.dumps({"FutureType":{"schema":{
                "$defs":definitions, "properties":{"choice":{key:branches}}
            }}}))
            result = subprocess.run([sys.executable, str(SCRIPT), "--choices-only", "--input", str(source),
                                     "--output", str(root / "output")], capture_output=True, text=True)
            self.assertEqual(result.returncode != 0, refused)
            if refused:
                self.assertIn("/schema/properties/choice", result.stderr)

    def test_rejects_nested_string_const_choices_with_optional_types(self):
        for branches in [
            [{"const": "a"}, {"const": "b"}],
            [{"type": "string", "const": "a"}, {"const": "b"}],
            [{"type": "string", "const": "a"}, {"type": "string", "const": "b"}],
        ]:
            with self.subTest(branches=branches), tempfile.TemporaryDirectory() as folder:
                root = Path(folder)
                source = root / "schemas.json"
                source.write_text(json.dumps({"FutureType": {"schema": {
                    "items": [{"properties": {"choice": {"oneOf": branches}}}]
                }}}))
                result = subprocess.run(
                    [sys.executable, str(SCRIPT), "--choices-only", "--input", str(source),
                     "--output", str(root / "output")], capture_output=True, text=True,
                )
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("FutureType", result.stderr)
                self.assertIn("/schema/items/0/properties/choice", result.stderr)

    def test_rejects_string_references_and_accepts_mixed_or_recursive_unions(self):
        for key in ["anyOf", "oneOf"]:
            for branches, refused in [
                ([{"$ref":"#/$defs/Text"}, {"type":"string", "pattern":"x"}], True),
                ([{"allOf":[{"$ref":"#/$defs/Text"}]}, {"enum":["x"]}], True),
                ([{"$ref":"#/$defs/Text"}, {"type":"null"}], False),
                ([{"$ref":"#/$defs/Cycle"}, {"type":"string"}], False),
            ]:
                self.assert_union(key, branches, {
                    "Text":{"type":"string"}, "Cycle":{"$ref":"#/$defs/Cycle"}
                }, refused)

    def test_rejects_untagged_wrapper_around_tagged_union(self):
        tagged = {"oneOf":[{"type":"object","properties":{"level":{"const":tag}},"required":["level"]}
                           for tag in ["a","b"]]}
        for key in ["anyOf", "oneOf"]:
            for branches, refused in [
                ([{"$ref":"#/$defs/Tagged"},{"type":"object"}], True),
                ([{"allOf":[{"$ref":"#/$defs/Tagged"}]},{"type":"object"}], True),
                ([tagged,{"type":"object"}], True),
                ([{"$ref":"#/$defs/Tagged"},{"type":"null"}], False),
                ([{"required":["min"]},{"required":["max"]}], False),
            ]:
                self.assert_union(key, branches, {"Tagged":tagged}, refused)
