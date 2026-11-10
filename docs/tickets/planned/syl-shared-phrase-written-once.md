---
needs: [cleanup-step6-surface]
---

# Write a phrase that a coordination shares once

## Why

A phrase that coordinated clauses share is printed once, and the term writes it once
(`docs/decisions/spelling.md` §3.3). The corpus writes it three other ways. *Arrest* copies it:
`cant(actor(enchanted(creature)), attack)` and `cant(actor(enchanted(creature)), block)` for
"can't attack or block". *Aether Tunnel* re-mentions it with an anaphor the card does not
print: `cant(any, block(it))` for "gets +1/+0 and can't be blocked". *Aim High* takes it as
`establishFor`'s subject and then writes `it` in each part. *Chub Toad* and `bushido` copy
`thisCreature` into both arms of an event `or`. A copy of a phrase that introduces a Referent
introduces it again, so *Predator's Strike* would get two targets and *Powerstone Minefield* a
creature per arm; neither card can be written correctly today.

## Design

- Where the shared phrase has the same role in every arm, the coordinated Actions stand under
  one Subject. An alternative event whose arms share their Subject is one `does` over an `or`
  of Actions ("Whenever a creature attacks or blocks"), and the event gate admits that `or` when
  every arm is an event-capable head (`docs/decisions/semantics.md` §4.6, which this ticket
  updates). A Handoff or a deontic rule takes coordinated Actions the same way ("Target creature
  gets +3/+3 and gains trample", "can't attack or block").
- Where the roles differ, the coordination's declaration takes the shared phrase as a parameter
  and each arm holds a placeholder word where the phrase belongs. The parameter is the binding
  (`docs/decisions/syl.md` §4). Spelling prints the phrase where the first arm's placeholder
  stands.
- No arm holds a copy of the shared phrase or an anaphor the card does not print.

## Open questions

- The placeholder word. `which` already stands for "the participant the nearest enclosing
  lookback describes", and an arm may itself hold a lookback, so reusing it would make its
  nearest binder ambiguous. Recommendation: one new word at `NounPhrase`, bound by the nearest
  enclosing coordination that takes a shared phrase; the owner names it.
- Which coordinations take a shared phrase. The cases found: the event `or` (*Chub Toad*,
  `bushido`, *Mirror Shield*'s "blocks or becomes blocked by this creature"), a conjunction of
  static parts (*Aether Tunnel*; `establishFor` already has the parameter and needs only the
  placeholder in place of `it`), and coordinated participles sharing a patient (*Cephalid
  Illusionist*'s "dealt to and dealt by target creature you control"). Recommendation: one
  declaration per sort that coordinates, added as a card needs it, starting with those three.
- "Can't attack or block" forbids each, so the `or` under `cant` means neither. Recommendation:
  write the printed `or` and let the deontic rule distribute over it, since the text says "or".

## Done when

- `docs/decisions/semantics.md` §4.6 states the event gate's rule for an `or` of Actions under
  one Subject, and `cargo xtask dsl check` enforces it.
- Every coordination in `plugins/` that copies a shared phrase into its arms, or re-mentions it
  with an unprinted anaphor, is rewritten one of the two ways; the landing lists each with the
  way it took. *Aim High*, *Aether Tunnel*, *Arrest*, *Chub Toad* and `bushido` are among them.
- *Powerstone Minefield*, *Predator's Strike*, *Mirror Shield* and *Cephalid Illusionist* are in
  `plugins/canon/cards/`, each written with its shared phrase once.
- That a Referent introduced by the one Subject of an `or` of Actions is reachable after the
  coordination ("deals 2 damage to it") is not checkable until reference resolution exists; the
  landing shows the term for *Powerstone Minefield*.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Joined trigger conditions ("When … and whenever …"): a disjunction of trigger conditions, each
  with its own trigger word, not an `or` of events with a shared phrase.
- A Template parameter mentioned in two arms of an `or` inside a body:
  `syl-binding-across-or-arms`.

## Pointers

`docs/decisions/spelling.md` §3.3; `docs/decisions/semantics.md` §4.1, §4.2, §4.6;
`plugins/builtin/primitives/event/or.syl`, `plugins/builtin/templates/action/establishFor.syl`,
`plugins/builtin/definitions/keyword_ability/bushido.syl`;
`plugins/canon/cards/{Aim High,Aether Tunnel,Arrest,Chub Toad}.syl`.
