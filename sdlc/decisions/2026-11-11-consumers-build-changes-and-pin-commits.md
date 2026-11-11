# Consumers build BioData changes and pin exact commits

Date: 2026-11-11. Ends the idle state of 2026-11-04-biodata-goes-idle for consumer-requested changes.

## Ruling

- Ian, 2026-11-11, on a consumer's request for a new parse form: BioData is "not officially 1.0 yet, so make whatever changes necessary."
- Ian, 2026-11-11: "no rush on publishing public biodata. just need to support internal ... dev and biomcp cli/mcp releases."
- Jarvis, 2026-11-11 (m33757), confirming for Ian: BioMCP's PM owns BioData's plan and coordination. "Additive, non-colliding consumer changes that pass BioData checks and fresh review can land with notice; coordinate behavior/layout changes and collisions first. Preserve second-consumer layout test; do not relax it without [that consumer's] owner agreement. Additional PM review is only for a named unresolved issue, not an automatic second gate."

## The working agreement

1. Each consumer pins an exact commit of this repository, never a tag or a branch. A change reaches a consumer only when that consumer moves its pin.
2. Commits to main and plain version tags are routine. A registry release, a hosted release page with notes, or an announcement needs Ian.
3. The consumer that needs a change builds it here, under `make gate` and a fresh reviewer. This repository keeps no standing team.
4. A change that only adds, passes the gate and has a fresh review lands with a notice to BioData's PM. A change to existing behaviour or layout, or one that collides with other open work, is ordered by BioData's PM first.
5. `crates/biodata/tests/hgvs_layout.rs` is not relaxed without the agreement of the consumer whose hot path it protects.
6. A refusal code that no longer applies is removed with no alias, as 2026-10-31-biodata-makes-no-backwards-compatibility-promises rules. A removed code's text is never reused for a different refusal.
7. Two consumers must be on the same pin before they exchange BioData records with each other.
8. Public text here names no private project.

## Options considered

A tagged release per change with Ian's approval each time (slow, and the tag adds nothing a commit hash lacks); or a standing BioData team (no steady work for one).
