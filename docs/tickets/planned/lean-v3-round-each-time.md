---
needs: [plugins-v3-spelling-conventions, semantics-rust-rewrite]
---
# "Round up each time." as a construct in its own position

Standard constraints apply. Lean first, in `lean/Semantics/`.

Six Vintage-legal cards end with "Round up each time." or "Round down each time.":

- *Peer into the Abyss*, *Fraying Omnipotence*, *Pox*, *Hydroid Krasis*, *Pox Plague*.
- *Saw in Half*: "… except their power is half that creature's power and their toughness is half
  that creature's toughness. Round up each time."

In every one, the fractions are all in the sentence just before it. Today *Saw in Half* writes
`half(up, …)` on each fraction. Spelling cannot put the shared sentence back from that, and the
only way out would be single-purpose Templates like `basePtHalfRoundedUpOf`.

## Decisions already made

- **Two constructions.** The inline ", rounded up/down" keeps its mode on its own fraction
  (`half(up, X)`). *Banshee* and *Aspect of Wolf* round two fractions of one sentence in
  different directions. "Round up/down each time." is a sentence-level construct placed after the
  sentence it covers, and the fractions there are written unrounded.
- **Fractions in general.** *Pox* says "a third", so the unrounded fraction covers more than
  `half`.
- **The check is structural; no discourse referent.** It works like `modifiesPrecedingAction`:
  - each "each time" construct covers the preceding sentence;
  - every fraction there must be unrounded, and at least one must exist;
  - an unrounded fraction that nothing covers is refused, because "the spell or ability will tell
    you whether to round up or down" [CR#107.1a].
- **No lowering work.** semantics_v3 is disconnected from other layers.

## Change

- **Lean:**
  - Add an unrounded fraction amount (half, a third) and the sentence-level rounding construct.
  - Checker: the three refusals above.
- **Templates:**
  - Spell the construct "Round <mode> each time.".
  - Spell the unrounded fraction "half <amount>" / "a third of <amount>", following
    `plugins-v3-spelling-conventions`.
- **Canon:** re-encode *Saw in Half*. Add *Pox* or *Fraying Omnipotence* as the multi-fraction
  witness.
- **ADR:** record the construct beside `modifiesPrecedingAction` in semantics-v3.

## Acceptance

- *Saw in Half* and the added witness check clean.
- Tests for:
  - an uncovered unrounded fraction;
  - an "each time" with no fraction before it;
  - a rounded fraction under an "each time".
- The `semantics` and `discourse` tests pass.
