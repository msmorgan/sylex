---
needs: [syl-single-face-card-call, cleanup-step6-spine, syl-spelling-optional-segments, syl-nominal-descriptions, syl-spelling-conventions]
---

# Zone words mean what they say, and prepositions stay out of `Location`

## Why

The `hand` and `graveyard` Templates are spelled "your hand" and "your graveyard" but their bodies
are unscoped (`zone(hand, none)`), while `yourHand` and `yourGraveyard` have the same spelling and
a scoped body. Callers use the unscoped pair to mean an owner's zone, any player's, or yours. An
object that would go to another player's hand, graveyard or library goes to its owner's instead
[CR#400.3], which is why a destination can be unscoped; no doc comment says so, and `zone`'s
`whose` parameter does not say whether it names an owner or a controller.

One `Location` sort also holds phrases of different kinds: bare zones ("the battlefield", "your
hand", `library` "its owner's library"), pile positions with their preposition (`onTop` "on top
of its owner's library", `onBottom` "on the bottom of its owner's library", `place` "the $end of
$pile"), pile positions without one (`topOf` "the top of $loc"), Position Phrases (`fromTop`
"$n~th from the top of $loc"), and a participle (`exiledWith` "exiled with $source"). The
declarations that take a `Location` add their own preposition: `putInto`'s doc comment reads
"put $subject into $destination (from $from)", `play`'s "(from $from)", and `cardIn` is spelled
"card in $zone". `putInto` and `play`'s `from` have no `spelling` yet, so the collisions below are
in the composition the declarations imply, not in a string anything prints:

- *Collected Company* `putInto(theRest, onBottom)`: "into on the bottom of its owner's library".
- *Brainstorm* `putInto(counted(exactly(2), cardIn(yourHand)), topOf(yourLibrary))`: "… card in
  your hand into the top of your library", where Oracle prints "two cards from your hand on top
  of your library".
- *Future Sight* and *Courser of Kruphix* `play(…, from = onTop)`: "from on top of its owner's
  library", where Oracle prints "from the top of your library".
- *Chronostutter* `putInto(target(isCreature), fromTop(2, library))`: "into second from the top
  of its owner's library", where Oracle prints "into its owner's library second from the top",
  the Position Phrase after the zone (`PositionDepictive`, `crates/english/grammar/english.fug`); *Approach of the Second Sun*'s `putInto(this, fromTop(7, library))` likewise, for
  "into its owner's library seventh from the top".

One collision is spelled today: `cardIn(exiledWith(…))` (*Cold Storage*, *Oblivion Ring*) composes
"card in exiled with this artifact".

"Outside the game" is no zone [CR#400.11], and `outsideTheGame` is a `Predicate` by design
(`plugins/builtin/templates/predicate/outsideTheGame.syl`). Where it is printed as an origin,
"from outside the game" (`companion.syl`, `learn.syl`), or coordinated with a zone ("Search your
library and/or outside the game", *Turtles Forever*; "your library, graveyard, and/or outside the
game", *Invasion of Arcavios*), nothing writes it. A contrasted origin (*Haakon, Stromgald
Scourge*'s "but not from anywhere else") has no written form either.

## Open questions

- **Where the preposition lives.** Options: `Location` holds bare zones and pile positions as
  noun phrases ("the top of your library"), and each verb's wording prints its own preposition,
  with a separate Template per printed preposition ("put … on top of", "put … into"); or the
  preposition belongs to the argument, in sorts for a move's destination and origin ("into your
  hand", "on top of your library", "from the top of your library"). Recommendation: the second,
  since "on top of" and "into" are printed with the same verb and the choice between them is the
  argument's, not the verb's.
- **"In $zone" and "exiled with $source" after a head.** With `syl-nominal-descriptions`, these
  are postmodifiers of a nominal ("a card in your graveyard", "cards exiled with this artifact").
  Recommendation: they are `PostModifier` declarations over a bare `Location`, and `exiledWith`
  leaves `Location`.
- **Position riders.** "Second from the top" follows the zone as a depictive of the move.
  Options: a rider on the move; or a destination form that prints the zone and the Position
  Phrase in that order. Recommendation: a rider, matching the Reading.
- **Outside the game as an origin.** Options: an origin form "from outside the game" beside the
  zone origins, leaving `outsideTheGame` a `Predicate` for "cards you own outside the game"; or a
  value at `Location`, which its doc comment and [CR#400.11] argue against. Recommendation: the
  origin form; how the engine holds such a card is `core-location-outside-the-game`'s.
- **Names.** `Destination` is already a type in `crates/core/src/action.rs` (a zone, or a library
  at an anchored position), with no glossary entry, and the glossary's **Location** is "a Zone, a
  position in a pile, anywhere, or a combination". Recommendation: the owner names the destination
  and origin sorts; if the destination sort is `Destination`, it gets a glossary entry that
  matches core's type, and **Location** is reworded to the bare place.

## Done when

- Each zone Template's spelling says what its body means, and no two share a spelling with
  different bodies; every caller in `plugins/` uses the one its printed words mean.
- An unscoped destination's doc comment cites [CR#400.3], and `zone`'s doc comment says what
  `whose` names.
- The open questions are answered, and the glossary says what each sort holds.
- No `Location` declaration's wording holds a preposition or participle that a declaration taking
  it also prints, and the cards above are written so that their composed wording matches Oracle
  text; a declaration whose wording cannot be stated yet keeps a `TODO(spelling)` naming it.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- An engine position for objects outside the game: `core-location-outside-the-game`.

## Pointers

- `plugins/builtin/templates/zone/`, `plugins/builtin/primitives/zone/`,
  `plugins/builtin/templates/action/putInto.syl`,
  `plugins/builtin/definitions/keyword_action/{play,sacrifice}.syl` (`play`'s origin, `sacrifice`'s
  destination), `plugins/builtin/templates/predicate/{cardIn,outsideTheGame}.syl`
- `plugins/canon/cards/{Collected Company,Brainstorm,Future Sight,Courser of Kruphix,Cold
  Storage,Oblivion Ring,Chronostutter,Approach of the Second Sun}.syl`
- Glossary **Location** (`docs/contexts/game-model/CONTEXT.md`), **Position Phrase**
  (`docs/contexts/oracle-english/CONTEXT.md`); `crates/core/src/action.rs` (`Destination`)
- `docs/evidence/cleanup-step6-syl/english-vs-syl-audit-verification.md` (§5)
