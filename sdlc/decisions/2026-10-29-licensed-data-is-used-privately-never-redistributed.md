# Licensed data is used privately and never redistributed

Date: 2026-10-29

Decided by Ian.

## Ruling

"Bio data can be public. I don't want to republish any of the GENIE information. We'll do it all internally: make private repos, make private experiments, whatever you got to do to prove out that the GENIE stuff works, to test the bio data. Bio data itself isn't about the GENIE data. We shouldn't be republishing it... Same thing with UMLS. Same thing with TCGA. None of this stuff should go into the bio data repo. They should just be used to create test cases and experiments, etc. Same thing with the ISCN nomenclature. We harvested examples from the ISCN book, but we converted them to new examples so that we weren't violating copyright. I still don't want to give those away. I still don't want to publish those and make people upset. We can have some test cases, but they got to be completely different than the ISCN test cases that are in the book. All that licensed material needs to stay private."

On the GENIE Data Model: "It's a reference only. You can use whatever data you want, just can't redistribute it."

## Reason

This repository becomes public. Licensed and controlled-access data, and cases derived from it or from copyrighted references, would be redistributed the moment they entered it.

## What it means

- Licensed and controlled-access data may be used for private tests, experiments and presentations of aggregate results. It never enters this repository, even as excerpts.
- The GENIE Data Model is a reference only. This repository copies none of its entities, attributes or value sets, and its element mapping stays private.
- ISCN cases in this repository are written fresh from the grammar rules and share no case with the book.

## Options considered

- Ask AACR for a commercial license to the GENIE Data Model. Declined: BioData needs the facts such models carry, not the model's own attributes.
- Keep small excerpts of licensed data as fixtures. Declined by the ruling.

## Replaces

Nothing. `AGENTS.md` and the replication corpus design already follow it.
