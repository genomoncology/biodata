"""Outside-in contract checks for fixture provenance."""

import hashlib
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

CHECK = Path(__file__).with_name("check-fixture-provenance.py")


class FixtureProvenance(unittest.TestCase):
    """Feature: fixture-provenance guards every published test data file."""

    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory()
        self.addCleanup(self.scratch.cleanup)
        self.root = Path(self.scratch.name)
        self.data = self.root / "cases" / "example.json"
        self.data.parent.mkdir()
        self.data.write_text('{"value": "example"}\n')

    def record(self, license="CC0-1.0", path="example.json", **changes):
        values = dict(license=license, path=path,
                      sha256=hashlib.sha256(self.data.read_bytes()).hexdigest())
        values.update(changes)
        origin = '' if license == 'synthetic' else 'origin = "https://example.org/data"\n'
        (self.data.parent / "provenance.toml").write_text(
            origin + f'license = "{values["license"]}"\n'
            'license_basis = "Public domain dedication"\n'
            'license_file = "LICENSE.txt"\n'
            '[[file]]\n'
            f'path = "{values["path"]}"\n'
            f'sha256 = "{values["sha256"]}"\n'
            'rows = 1\ntransformation = "Created for testing"\n')
        (self.data.parent / "LICENSE.txt").write_text("License notice\n")

    def check(self, success, message):
        result = subprocess.run([sys.executable, str(CHECK), str(self.root)],
                                capture_output=True, text=True)
        self.assertEqual(result.returncode, 0 if success else 1, result.stderr)
        self.assertIn(message, result.stdout + result.stderr)

    def test_listed_cc0_file_passes(self):
        self.record()
        (self.root / "test.rs").write_text("// Rust source is not fixture data\n")
        self.check(True, "1 data files")

    def test_synthetic_file_needs_no_origin_url(self):
        self.record("synthetic")
        self.check(True, "1 data files")

    def test_unlisted_file_fails_and_names_it(self):
        self.check(False, "no provenance: cases/example.json")

    def test_changed_file_fails_on_its_hash(self):
        self.record()
        self.data.write_text("changed\n")
        self.check(False, "hash mismatch: cases/example.json")

    def test_unapproved_license_fails_and_names_it(self):
        self.record("CC-BY-4.0")
        self.check(False, "unapproved license: CC-BY-4.0")

    def test_record_for_missing_file_fails(self):
        self.record(path="missing.json")
        self.check(False, "missing file: cases/missing.json")

    def test_record_paths_cannot_escape_their_folder(self):
        self.record(path="../outside.json")
        self.check(False, "path outside record folder")

    def test_nested_unlisted_data_is_not_hidden_by_a_parent_record(self):
        self.record()
        nested = self.data.parent / "nested"
        nested.mkdir()
        (nested / "extra.json").write_text("{}")
        self.check(False, "no provenance: cases/nested/extra.json")

    def test_malformed_record_fails_with_a_diagnostic(self):
        (self.data.parent / "provenance.toml").write_text("license = [")
        self.check(False, "fixture provenance:")

    def test_nonsynthetic_record_needs_origin(self):
        self.record()
        record = self.data.parent / "provenance.toml"
        record.write_text(record.read_text().replace('origin = "https://example.org/data"\n', ''))
        self.check(False, "missing origin")

    def test_symlink_cannot_hide_unchecked_data(self):
        self.record()
        (self.root / "linked").symlink_to(self.data.parent, target_is_directory=True)
        self.check(False, "symlink is not fixture data: linked")

    def test_default_root_must_be_a_directory_if_present(self):
        checkout = self.root / "checkout"
        scripts = checkout / "scripts"
        scripts.mkdir(parents=True)
        command = scripts / CHECK.name
        shutil.copyfile(CHECK, command)
        tests = checkout / "crates" / "biodata" / "tests"
        tests.parent.mkdir(parents=True)
        tests.write_text("unrecorded data")
        result = subprocess.run([sys.executable, str(command)],
                                capture_output=True, text=True)
        self.assertEqual(result.returncode, 1, result.stdout)
        self.assertIn("test root is not a directory", result.stderr)

    def test_fixture_root_cannot_be_a_symlink(self):
        target = self.root / "outside"
        target.mkdir()
        self.root = self.root / "linked"
        self.root.symlink_to(target, target_is_directory=True)
        self.check(False, "test root cannot be a symlink")


if __name__ == "__main__":
    unittest.main()
