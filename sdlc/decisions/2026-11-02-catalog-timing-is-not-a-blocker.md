# The catalog timing gap is not a blocker

Date: 2026-11-02

Decided by Ian.

## Ruling

"Actually, I don't care. 1.5% isn't worth it. Consider it a solved problem." (Ian to the second consumer's PM, 2026-11-02.)

"I told the [second consumer's] team I don't care about the 1% or 2% problem, so just stop worrying about that." (Ian to the BioData PM, 2026-11-02.)

## Reason

The second consumer's timing showed that the 6 to 8% step came from how rustc splits the crate into its default 16 codegen units, not from new work per row: with one codegen unit the step vanished, and every pin ran about 16% faster. The residual 1.3 to 1.6% sits in commits the parse path never calls. The second consumer moves its release build to one codegen unit.

## What it means

- No pin waits on catalog timing. 08d9399 lands as qualified, with no inline hints, bisect or further timing.
- It replaces the 2026-11-02 PM decision on ticket 0006 that no pin lands above a 1% catalog timing bar.

## Options considered

- Keep the 1% bar and keep hunting the residual. Rejected by Ian.
- Add `#[inline]` hints to restore inlining under 16 codegen units. Two hint attempts missed, and the in-place binary comparison found no helper shortlist.
