---
needs: [cleanup-step6-surface, english-shared-complement-through-predicative]
specs: [voice-and-object-phrases]
---

# Write a phrase that coordinated verb phrases share once

## Why

A phrase that coordinated phrases share is printed once, and the term writes it once
(`docs/design/spelling.md` §3.3). A copy of a phrase that introduces a Referent introduces it
again, and an anaphor the card does not print is a word the text does not have. Two kinds of
card still cannot be written that way:

- **One subject over coordinated verb phrases.** *Chub Toad* ("Whenever this creature blocks or
  becomes blocked") and `bushido` write `this(creature)` into both arms of an event `or`. *Powerstone
  Minefield* ("Whenever a creature attacks or blocks, this enchantment deals 2 damage to it.")
  would get a creature per arm, and *Predator's Strike* ("Target creature gets +3/+3 and gains
  trample until end of turn.") two targets. Each is one subject over an `and` or `or` of verb
  phrases, with no new mechanism (spec §5, §8).
- **A noun phrase that coordinated verb phrases share after them.** *Bounding Krasis* prints "you
  may tap or untap target creature", *Dream Fighter* "Whenever this creature blocks or becomes
  blocked by a creature", *Mirror Shield* grants "Whenever a creature with deathtouch blocks or
  becomes blocked by this creature, destroy that creature.", and *Cephalid Illusionist* prints
  "Prevent all combat damage that would be dealt to and dealt by target creature you control this
  turn." No word writes the shared phrase once after the arms (spec §7).

Written against the surface (`cleanup-step6-surface`), which has clauses, the passive words, the
deontic operators and `and` and `or` at `VerbPhrase`.

## Open questions

- **Whether any clause-level coordination shares a phrase.** `shared` is declared for verb
  phrases sharing a noun phrase, and a second sort gets its own declaration when a card needs it
  (spec §7). Five units in `data/derived/english-forests.sqlite` have a Reading built by
  `Coordination:Clause/shared` (on *Mishra, Claimed by Gix*, *Varchild, Betrayer of Kjeldor*,
  *The Celestial Toymaker*, *Heaped Harvest* and *Snow Devil*), and none has been read to see
  whether that Reading is the one its card means. Recommendation: read the five; a card that
  means the shared Reading gets a declaration at `Clause`, with that card as its caller, and the
  landing says so for each.

## Done when

- The open question is answered.
- `shared(arms: VerbPhrase, rhs: NounPhrase): VerbPhrase`, worded "$arms $rhs", and `_` at
  `NounPhrase`, which prints no word, exist. `cargo xtask dsl check` refuses a `shared` whose
  `arms` is not a coordination, an arm that does not hold exactly one `_`, and a `_` anywhere
  else, with a negative test for each, and the event gate admits a `shared` over event phrases,
  with a test.
- *Bounding Krasis*, *Dream Fighter*, *Mirror Shield* and *Cephalid Illusionist* are in
  `plugins/canon/cards/`, each written with its shared phrase once through `shared` and checked
  against its Oracle text. *Dream Fighter*'s event is the spec's §7 term, over `block` and
  `becomeBlocked(by = _)`, and *Aisling Leprechaun* and *Inferno Elemental*, which print the same
  wording, write it the same way.
- *Powerstone Minefield* and *Predator's Strike* are in `plugins/canon/cards/`, each one subject
  over a coordination of verb phrases, checked against its Oracle text.
- No coordination in `plugins/` copies a shared phrase into its arms or re-mentions it with an
  anaphor the card does not print; each one found is rewritten as one subject over a
  coordination or through `shared`, and the landing lists each with the way it took. *Chub
  Toad*, `bushido` and *Aim High* are among them.
- No coordination at another sort takes a shared phrase. Each fixed wording in the spec's §7
  table at a sort other than `VerbPhrase` that a canon card prints ("less than or equal to",
  "base power and toughness") is a Template with an ordinary parameter; the landing lists each
  with its caller.
- The landing shows the term for *Powerstone Minefield*, whose "deals 2 damage to it" reads the
  Referent that the one subject of an `or` of verb phrases introduces. Reference resolution, which
  would check that reading, does not exist yet.
- `docs/design/spelling.md` §3.3 and §10 and `docs/design/architecture.md` describe a shared
  phrase as the spec's §7 does and no longer name Factoring. **Factoring** is retired from
  `docs/contexts/oracle-english/CONTEXT.md`, and **Shared Phrase** is a new entry in
  `docs/contexts/game-model/CONTEXT.md`.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Joined trigger conditions ("When … and whenever …"): a disjunction of trigger conditions, each
  with its own trigger word, not an `or` of events with a shared phrase.
- A Template parameter mentioned in two arms of an `or` inside a body:
  `syl-binding-across-or-arms`.
- Two determiners sharing a nominal (*Balthor the Defiled*'s "all black and all red creature
  cards"): a Template or a declaration of its own, which waits on
  `english-shared-nominal-determiner-coordination`.
- A participle that describes a state (*Abu Ja'far*'s "all creatures blocking or blocked by
  it"): not designed (spec §11).
- How sentences join, and the wordings of `sequentially`: `syl-sequentially-wordings`.

## Pointers

- `docs/specs/voice-and-object-phrases.md` §5, §7, §8, §9, §10
- `docs/design/spelling.md` §3.3, §10; `docs/design/semantics.md` §3.2;
  `docs/design/surface.md` §7.5
- `plugins/builtin/primitives/event/or.syl`,
  `plugins/builtin/definitions/keyword_ability/bushido.syl`,
  `plugins/builtin/templates/action/becomeBlocked.syl`
- `plugins/canon/cards/{Aim High,Chub Toad,Aisling Leprechaun,Inferno Elemental}.syl`
- `crates/xtask/src/dsl_check.rs`
