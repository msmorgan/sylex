---
needs: [syl-printed-it-self-references, syl-argument-naming, syl-condition-first-static, syl-lookback-point-from-trigger, syl-single-face-card-call, syl-implicit-coercions, syl-number-snapshot, syl-self-name-references, syl-underscore-file-names]
---

# One `this(head)` replaces the 21 `this<Head>` Templates and bare `this`

## Why

`plugins/builtin/templates/noun/` holds 21 `this<Head>` Templates (`thisCreature`, `thisAura`,
`thisSpell`, `thisPermanent`, …), each `headed(<head>, this)` for one head noun over the `this`
primitive (`plugins/builtin/primitives/pronoun/this_.syl`). Three have no caller (`thisClass`,
`thisEmblem`, `thisSpacecraft`), and a head with no Template can't be written at all. Callers also
mix `thisSpell`, `thisPermanent` and bare `this` for the same printed words.

Bare `this` is written about 122 times in 71 `plugins/builtin/` files (66 in keyword abilities,
29 in card types) and 90 times in 75 canon cards, but the rules name the source with a head:
"this card" (cycling's "Discard this card" [CR#702.29a]), "this creature", "this spell" or "this
object" (changeling's "This object is every creature type" [CR#702.73a]). `changeling.syl` and
`cycling.syl` write bare `this`. Oracle does print a bare "this" in champion's reminder text
("When this enters"), but champion's rule says "this permanent" [CR#702.72a].
It lands after `syl-printed-it-self-references`, which rewrites about 99 of the same call sites
to `it`, so those sites are rewritten once and the two tickets never edit them in parallel.

Its `this<Head>` sites also sit in calls three tickets rewrite first: `syl-argument-naming`
(83 arguments it names or unnames, in 79 files), `syl-condition-first-static` (the
condition-first statics, such as *Guardian Beast*'s `is(thisCreature, isUntapped)`) and
`syl-lookback-point-from-trigger` (the `move` and `putIntoFromBattlefield` calls it respells,
such as `aura.syl`'s state-based row and *Rancor*'s trigger).

With the card-type and subtype coercions of `syl-implicit-coercions`, a caller passes such a head
bare (`creature`, `aura`), so that ticket lands first.

`syl-self-name-references` rewrites the canon `this` sites that print the card's name first,
and leaves the "this spell" sites (*Abolish*'s `altCost(this, …)`, *Embercleave*'s
`costShift(this, …)`, *Conqueror's Pledge*'s `keywordCostPaid(kicker, this)`) to this ticket.
`syl-underscore-file-names` renames `this_.syl`, which this ticket rewrites, and lands first.

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

## Design

- `this(head: NounWord)` takes the name `this`, and nullary `this` is retired.
- Every bare `this` becomes `this(<head>)` with the head its CR or Oracle text prints:
  card-zone keywords and conferrals take `this(card)` (cycling [CR#702.29a]); type and permanent
  rules take `this(creature)`, `this(planeswalker)`, `this(battle)` or `this(permanent)`; and
  zone-independent ones take `this(object)` (changeling [CR#702.73a]).
- `object` is a new `NounWord`, the CR's object [CR#109.1].

## Done when

- `this(head)` replaces the 21 files, and every builtin and canon caller uses it with the head
  its text prints; where a caller writes `thisSpell`, `thisPermanent` or bare `this`, it now
  matches the printed words, the canon "this spell" sites above included.
- No bare `this` remains anywhere in `plugins/`, and the landing lists `headed`'s remaining
  callers.
- Declaration order stays the printed order (`syl.md` §5).
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Mentions where the card prints "it": `syl-printed-it-self-references`.
- Mentions where the card prints its name: `syl-self-name-references`.

## Pointers

`plugins/builtin/templates/noun/this*.syl`, `plugins/builtin/primitives/pronoun/this_.syl`,
`docs/decisions/syl.md` §3.
