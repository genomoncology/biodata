# BioData makes no backwards compatibility promises

Date: 2026-10-31

Relayed by BioMCP's PM on 2026-11-01.

## Ruling

Ian, 2026-10-31: "The CLI and MCP for now in BioMCP has no backwards compatibility promises." BioData follows the same rule: no aliases for old names and no compatibility code.

## Reason

BioData 1.0 is the first public release of the merged crate. Aliases kept for old spellings widen every parser and schema and keep code that no user of 1.0 needs.

## What it means

- BioData's parsers and schemas accept one spelling per value. An old or alternate spelling kept only for compatibility is removed, and the reconciliation record lists each removal.
- The first removal is the `semanticscholar:` author ID prefix beside `semantic-scholar:`, which the second consumer does not use.
- Ticket 0006's rule that every existing case serializes byte-identically still holds. A removal changes what a parser accepts, never what an accepted value serializes to.

## Options considered

- Keep aliases through 1.0 and drop them later. Rejected: 1.0 is the point where the public contract starts, so an alias kept now becomes a promise.
