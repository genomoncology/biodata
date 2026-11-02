# Build the private ISCN overlap check in the private test repository. It reads every ISCN case in the second consumer's crates/biodata/tests/cases, the public and private folders both, from that consumer's checkout, and fails when an ISCN string in this repository matches one. It copies no case.

Status: closed.
Promoted: 0001

Kind: idea
When: when pm enrolls the private test repository under team biodata, before 0006 adds any ISCN case

## Problem

Build the private ISCN overlap check in the private test repository. It reads every ISCN case in the second consumer's crates/biodata/tests/cases, the public and private folders both, from that consumer's checkout, and fails when an ISCN string in this repository matches one. It copies no case. Keeps book-derived ISCN cases out of the public crate, as the licensed-data ruling requires.
