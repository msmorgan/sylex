---
needs: [plugins-v3-spelling-conventions, lean-retirement]
---
# "Round up each time." as a construct in its own position

Standard constraints apply. Done in `.syl` and Rust (`semantics`, `discourse`) after
`semantics-rust-rewrite`; the Lean names below are the current AST, which that epic ports.

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
  different directions. "Round up/down each time." is a sentence-level construct, and the fractions
  it covers are written unrounded.
- **The construct wraps the sentence it covers:** `roundEachTime(mode, action)`, spelled as that
  sentence followed by "Round <mode> each time.". It cannot be a trailing step that looks back at
  "the preceding step". *Pox* and *Fraying Omnipotence* cover a "then" chain of three or four
  Actions in one sentence. As steps of one `sequentially`, a look-back at the preceding step would
  miss all but the last fraction.
- **Fractions in general.** *Pox* says "a third", so the unrounded fraction covers more than
  `half`.
- **The check is structural; no discourse referent:**
  - every fraction inside the wrapped Action must be unrounded, and at least one must exist;
  - an unrounded fraction that no construct wraps is refused, because "the spell or ability will
    tell you whether to round up or down" [CR#107.1a].
- **No lowering work.** semantics_v3 is disconnected from other layers.

## Change

- **AST (Rust `semantics`):** add an unrounded fraction amount (half, a third) and the wrapping
  rounding construct.
- **`discourse`:** the refusals above.
- **Templates:**
  - Spell the construct as its wrapped sentence followed by "Round <mode> each time.".
  - Spell the unrounded fraction "half <amount>" / "a third of <amount>", following
    `plugins-v3-spelling-conventions`.
- **Canon:** re-encode *Saw in Half*. Add *Pox* or *Fraying Omnipotence* as the witness for a
  "then" chain.
- **ADR:** record the construct in `semantics.md`.

## Acceptance

- *Saw in Half* and the added witness check clean.
- Tests for:
  - an unrounded fraction that nothing wraps;
  - an "each time" that wraps no fraction;
  - a rounded fraction inside an "each time";
  - every fraction of a "then" chain being covered.
- The `semantics` and `discourse` tests pass.
