"""Exercise reconciliation ownership checks with synthetic public items."""
import contextlib
import importlib.util
import io
import unittest
from pathlib import Path

SPEC = importlib.util.spec_from_file_location(
    "reconciliation", Path(__file__).with_name("check-reconciliation.py"))
CHECK = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECK)


def record(rows):
    return """
## Rules
| Rule | Meaning |
|---|---|
| MOVE | One source. |
| ONE | Both sources. |
| ONE-W | Wider wire. |
| NAME | Separate meanings. |

## Items
| BioMCP | Second consumer | Rule | Note | Merged |
|---|---|---|---|---|
""" + "\n".join(rows) + "\n"


class ReconciliationOwnership(unittest.TestCase):
    def check(self, rows, actual):
        with contextlib.redirect_stdout(io.StringIO()):
            return CHECK.check_merged(record(rows), actual)

    def test_one_row_can_merge_two_source_items(self):
        self.assertFalse(self.check([
            "| `Value` in `src/first.rs` | `Value` in `src/second.rs` | ONE | | `Value` in `src/value.rs` |",
        ], {("Value", "src/value.rs"): "struct"}))

    def test_duplicate_source_row_is_refused(self):
        row = "| `Value` in `src/value.rs` | none | MOVE | | `Value` in `src/value.rs` |"
        with self.assertRaisesRegex(SystemExit, "more than one row"):
            self.check([row, row], {("Value", "src/value.rs"): "struct"})

    def test_distinct_source_rows_cannot_claim_one_merged_location(self):
        self.assertTrue(self.check([
            "| `Value` in `src/first.rs` | none | MOVE | | `Value` in `src/value.rs` |",
            "| none | `Value` in `src/second.rs` | MOVE | | `Value` in `src/value.rs` |",
        ], {("Value", "src/value.rs"): "struct"}))

    def test_recorded_location_must_exist(self):
        self.assertTrue(self.check([
            "| `Value` in `src/value.rs` | none | MOVE | | `Value` in `src/missing.rs` |",
        ], {("Value", "src/value.rs"): "struct"}))


if __name__ == "__main__":
    unittest.main()
