---
needs: [syl-spelling-conventions, lean-retirement, cleanup-step6-surface]
---

# Rounding is printed where it is: "Round up each time." and ", rounded up"

## Why

Six Vintage-legal cards end a sentence of fractions with "Round up each time." or "Round
down each time.": *Peer into the Abyss*, *Fraying Omnipotence*, *Pox*, *Hydroid Krasis*, *Pox
Plague* and *Saw in Half*. `Saw in Half.syl` writes `half(up, …)` on each fraction, so spelling
cannot give back the shared sentence, and the only way out would be single-purpose Templates
like `basePtHalfRoundedUpOf`.

The inline ", rounded up" or ", rounded down" is a different construction and stays on its own
fraction: *Banshee* and *Aspect of Wolf* round two fractions of one sentence in different
directions. `half` (`plugins/builtin/primitives/quantity/half.syl`) spells it inside the amount,
"half $amount, rounded $rounding", but no Reading puts it there. `cargo xtask english parse`
attaches it as a supplement to three different hosts, never inside the amount's noun phrase:

- in a verb's complements, before the next one: *Banshee*'s "deals half X damage, rounded down,
  to any target, and half X damage, rounded up, to you" (`InterleavedSupplement`);
- in a cost: *Lurking Evil*'s "Pay half your life, rounded up:" (`SupplementedVerbalCost`);
- at the end of a clause, after a ", where X is …" supplement: "Create X 2/2 black Zombie creature
  tokens with decayed, where X is half the number of creature cards in your graveyard, rounded
  up." (`ParticipialSupplement`, over the whole clause and its "where" supplement).

## Design

- An unrounded fraction amount exists for half and for a third (*Pox*), spelled "half $amount"
  and "a third of $amount".
- `roundEachTime(action, mode)` wraps the run of sentences it covers and is spelled as that run
  followed by "Round $mode each time.", its parameters in that printed order (`syl.md` §5). Its
  `action` Hole holds a run, `$action*` (`spelling.md` §2, §7): *Saw in Half*'s holds the two sentences before
  the rounding sentence, and *Hydroid Krasis*'s the one. Its own phrase is a run too, so `when`'s
  effect Hole holds it, and the trigger clause printed in its first sentence governs it by the run
  rule. It wraps the whole run, so a "then" chain of several Actions (*Pox*, *Fraying
  Omnipotence*) is covered; it is not a trailing step that looks back at the step before it.
- The rounded `half` keeps one expansion and prints its rounding as a Tail: one Template per host
  the Readings give (the interleaved complement, the cost, the clause), each its own wording.

## Open questions

- **The Tails' names and placement.** Recommendation: named for their host; the placement among
  the host's other adjuncts follows `syl-tail-placement`, which lands first through the surface
  gate.

## Done when

- The open questions are answered and written in `docs/decisions/spelling.md` and the
  declarations' doc comments.
- The checks are structural: every fraction inside the wrapped Action is unrounded and at least
  one exists; an unrounded fraction that no `roundEachTime` wraps is refused, because the spell or
  ability must say which way to round [CR#107.1a]. Each refusal has a test: an unrounded fraction
  nothing wraps, an "each time" with no fraction, a rounded fraction inside an "each time".
- *Saw in Half* is re-encoded with `roundEachTime`, and *Pox* or *Fraying Omnipotence* is added as
  the witness for a "then" chain; a test shows every fraction of that chain is covered.
- `half`'s spelling prints no rounding inside the amount, and every rounded `half` in `plugins/`
  calls the Tail of its printed host, each checked against Oracle text; *Banshee* and *Lurking
  Evil* are in `plugins/canon/cards/` as the interleaved and cost witnesses.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/quantity/half.syl`, `plugins/canon/cards/Saw in Half.syl`;
`crates/english/grammar/english.fug` (`InterleavedSupplement`, `SupplementedVerbalCost`,
`ParticipialSupplement`, `SentenceSequence`).
