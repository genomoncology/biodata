# Working on BioData

Read the ticket you build and `sdlc/plan.md`. Rulings live in `sdlc/decisions/`.

## Boundary

BioData holds typed values, their parsers and validators, and their JSON schemas. It makes no network call, holds no credential, reads no database and makes no clinical interpretation. Consumers own retrieval, storage and product behavior.

## Public text

This repository becomes public. Public text, tests, commit messages and diagnostics name no private project, machine or credential. Call a private consumer "the second consumer". Fixtures come from synthetic data or sources whose license allows republishing with attribution. Licensed or controlled-access data, and cases derived from it or from copyrighted references, never enter this repository, even as excerpts. Each fixture records its origin and license. Code from another repository arrives as a snapshot, never with its history.

## Gates

`make gate` runs formatting, lint and tests. Run it through `pm job` before review.

## Value contracts

Values parse without interpretation. Parsers return typed refusals with stable codes and retain no rejected input. Fields stay private behind validating constructors unless a plain record carries no invariant.
