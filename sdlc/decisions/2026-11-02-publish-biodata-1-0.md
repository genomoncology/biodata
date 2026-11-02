# Publish BioData 1.0

Date: 2026-11-02

Decided by Ian.

## Ruling

"You can also mark things as public whenever you're ready. It can become bio data. That's fine. Go ahead and clean up the repo, and then tell [the workspace coordinator] about any repos you need cleaned up or deleted." (Ian to the BioData PM, 2026-11-02.) BioMCP's PM relayed the same approval the same day: rename this repository to `genomoncology/biodata`, make it public and tag `v1.0.0`.

## What it means

- "Yeah, you can publish it with a fresh history." (Ian, 2026-11-02, via Jarvis.) The public `genomoncology/biodata` starts with one root commit whose tree is main's at the time of publication, with product files byte-identical to `08d9399cc0a29d36fd48c002ccec57a34df37f09`, the commit both consumers qualified. `v1.0.0` tags that root commit. This private repository stays as it is, with its full history, as the backup.
- The old private repository that holds the `genomoncology/biodata` name moves to the legacy repository name and stays private, so this one can take the name. Archiving or deleting it, and any local folder renames, go to the workspace coordinator for Ian.
- Before the visibility change, the current files on main name no private project, and every ref the remote advertises passes a secret scan over its whole history. A private project name in an old commit may stay: "the documentation of bio data shouldn't refer to the [second consumer's] project, but it's okay if it's in the git history" (Ian, 2026-11-02, with the private name in his words replaced as [the second consumer]). Every remote branch except main is deleted, and pm's own refs stay.
- A crates.io release still waits for Ian.

## Replaces

Nothing. It carries out the 2026-10-28 PM decision on ticket 0006 that this repository takes that name and becomes public before BioMCP cuts its release candidate.
