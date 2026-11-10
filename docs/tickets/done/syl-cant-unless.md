---
needs: []
---

# A deontic rule's "unless" is its own modifier, read inside it

## Why

[CR#205.4e] prints "A player can't cast a legendary instant or sorcery spell unless that player
controls a legendary creature or a legendary planeswalker". `legendary` wrote it as
`onlyUnless(cant(…), …)`, whose condition is a conditional's antecedent: §2.2 item 2 of
`semantics.md` gives the antecedent no access to the `cant`'s Subject, so neither "a player" nor
"that player" could be written and `actorControls` stood in. *Pithing Needle*, *Damping Matrix*
and read ahead ([CR#702.155a]) folded their printed "unless" into the prohibited abilities'
predicate.

## Design

- `DeonticMods` has an optional `unless` Condition; `cant` and `must` take it as their last
  parameter, `unless`. Cards print "can't … unless" (*Pithing Needle*) and "… if able unless"
  (*Reckless Cohort*). `doesntUntap` passes its own `unless` to the prohibition it expands to.
- `can` takes none: the one card that prints "can … unless", *Chaos Lord*, also prints "if the
  number of permanents is even", and no Condition word tests an amount's parity.
- The Unless Condition is read inside the rule's box, after its Subject and Action, so it reaches
  the Subject's noun phrase and the patients, a hypothetical indefinite among them; the rule
  applies to each participant exactly when the condition, read of that participant, does not hold
  (`semantics.md` §2.2 item 3, §4.7).
- `onlyUnless` stays for its callers that are not deontic rules.

## Done when

- `semantics.md` §2.2 item 3, §2.4 item 2 and §4.7 and the game-model glossary state the Unless
  Condition, its distributive reading and the universal force of a hypothetical indefinite under
  `cant`.
- `legendary`, the creature card type's two [CR#302.6] restrictions, read ahead, *Pithing
  Needle*, *Damping Matrix* and *Bombur, Gentle Dreamer* write their printed "unless" as the
  rule's `unless`; `legendary` writes "A player" and "that player", and no `isPermanent` the rule
  does not print. *Reckless Cohort* is in canon, its "unless" the `must`'s.
- `syl-conditional-wording-mismatches` no longer lists what this fixes.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask`, `cargo clippy -p
  xtask --tests -- -D warnings`, `cargo xtask cite check` and `kata kanban check` pass.

## Pointers

`docs/decisions/semantics.md` §2.2, §4.7; `plugins/builtin/templates/static/{cant,must}.syl`;
`plugins/builtin/templates/ability/{onlyUnless,doesntUntap}.syl`;
`crates/xtask/src/dsl_check/schema.rs`.
