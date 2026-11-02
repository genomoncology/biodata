# BioData holds pure content

Date: 2026-11-01

Decided by Ian.

## Ruling

"Basically biodata is the pure form of content. Adapters for API calls, pagination, indexing, database ORM etc are outside of biodata."

## Reason

BioData is the universal core every consumer shares. Each consumer fetches, pages, indexes and stores content its own way, so only the content itself is common to all of them.

## What it means

- A type belongs in BioData when it states content: an entity, an identifier, a coordinate, a notation, a vocabulary, or a publisher's record of an entity.
- A type stays in its consumer when it carries a request, a reply envelope, a page or cursor, an index or position, a fetch or snapshot status, a source selector, or a database mapping.
- A consumer type that adds one of those concerns to BioData content wraps the BioData type and does not copy it. For example, a row that adds a native index to a BioData adverse event record stays a consumer wrapper around the BioData record.
- This sharpens the Boundary in `CLAUDE.md` and BioMCP's own 2026-10-23 split. It decides ticket 0015's line between a candidate and a consumer type.
