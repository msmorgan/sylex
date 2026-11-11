---
needs: [cleanup-step6-surface]
---

# Write a phrase that a coordination shares once

## Why

A phrase that coordinated clauses share is printed once, and the term writes it once
(`docs/decisions/spelling.md`, on shared material at a coordination). The corpus writes it three other ways. *Arrest* copies it:
`cant(actor(enchanted(creature)), attack)` and `cant(actor(enchanted(creature)), block)` for
"can't attack or block". *Aether Tunnel* re-mentions it with an anaphor the card does not
print: `cant(any, block(it))` for "gets +1/+0 and can't be blocked". *Aim High* takes it as
`establishFor`'s subject and then writes `it` in each part. *Chub Toad* and `bushido` copy
`thisCreature` into both arms of an event `or`. A copy of a phrase that introduces a Referent
introduces it again, so *Predator's Strike* would get two targets and *Powerstone Minefield* a
creature per arm; neither card can be written correctly today.

`docs/decisions/semantics.md` contradicts `spelling.md`'s shared material at a coordination twice. §4.6 writes "attacks or
blocks" as `or([does(actor(x), attack), does(actor(x), block)])`, each arm a whole event with its
own subject, and says "An `or` over Actions would be a compound action, which the gate refuses";
`spelling.md` writes it as one `does` over an `or` of the Actions. And §4.2 makes "becomes blocked" its own
event head, `becomeBlocked` (the Blocked status beginning, once per combat, even with no blocker
[CR#509.1h,509.3c]), not the agentless passive of `block`, while `spelling.md` gives *Chub Toad*'s "blocks
or becomes blocked" as one subject standing as the performer of one event and the patient of the
other.

*Predator's Strike*'s one subject over "gets +3/+3 and gains trample" is written against
`syl-continuous-effect-subject`'s declarations, which take no subject.

## Design

- Where the shared phrase has the same role in every arm, the coordinated Actions stand under
  one Subject. An alternative event whose arms share their Subject is one `does` over an `or`
  of Actions ("Whenever a creature attacks or blocks"), and the event gate admits that `or` when
  every arm is an event-capable head (`docs/decisions/semantics.md` §4.6, which this ticket
  updates). A Handoff, a deontic rule and the declaration that gives a continuous effect its
  subject (`syl-continuous-effect-subject`) take coordinated phrases the same way ("that player
  copies it and may choose new targets", "can't attack or block", "Target creature gets +3/+3
  and gains trample").
- Where the roles differ, the coordination's declaration takes the shared phrase as a parameter
  and each arm holds a placeholder word where the phrase belongs. The parameter is the binding
  (`docs/decisions/syl.md` §4). Spelling prints the phrase where the first arm's placeholder
  stands.
- No arm holds a copy of the shared phrase or an anaphor the card does not print.

## Open questions

- **What "becomes blocked" is in *Chub Toad*'s coordination.** Options: (a) `semantics.md` §4.2
  stands: `becomeBlocked` is its own event head whose one argument is its patient
  (`does(any, becomeBlocked(x))`), so "blocks or becomes blocked" is still a coordination whose
  shared subject has different roles, the performer of `block` and the patient of
  `becomeBlocked`, and `spelling.md` names `becomeBlocked` as the event whose patient the shared
  subject is; (b) "becomes blocked" is the agentless passive of `block`, which would trigger
  once per blocker, against [CR#509.3c] ("generally triggers only once each combat for that
  creature, even if it's blocked by multiple creatures"). Recommendation: (a).

- The placeholder word. `which` already stands for "the participant the nearest enclosing
  lookback describes", and an arm may itself hold a lookback, so reusing it would make its
  nearest binder ambiguous. Recommendation: one new word at `NounPhrase`, bound by the nearest
  enclosing coordination that takes a shared phrase; the owner names it.
- Which coordinations take a shared phrase. The cases found: the event `or` (*Chub Toad*,
  `bushido`, *Mirror Shield*'s "blocks or becomes blocked by this creature"), a conjunction of
  static parts (*Aether Tunnel*; `establishFor` already has the parameter and needs only the
  placeholder in place of `it`), and coordinated participles sharing a patient (*Cephalid
  Illusionist*'s "dealt to and dealt by target creature you control"), and determined phrases
  sharing their nominal (*Balthor the Defiled*'s "all black and all red creature cards", two
  determiners and premodifiers over one printed "creature cards", which waits on the Reading
  `english-shared-nominal-determiner-coordination` adds). Recommendation: one
  declaration per sort that coordinates, added as a card needs it, starting with the first three.
- "Can't attack or block" forbids each, so the `or` under `cant` means neither. Recommendation:
  write the printed `or` and let the deontic rule distribute over it, since the text says "or".

## Done when

- `docs/decisions/semantics.md` agrees with `spelling.md` on shared material at a coordination: §4.6 writes "attacks or blocks"
  as one `does` over an `or` of Actions, no longer says the gate refuses an `or` over Actions, and
  states that the event gate admits such an `or` when every arm is an event-capable head (a
  Deed's Action or a keyword-action Definition matched by its Deed) and refuses it otherwise; §4.2
  and `spelling.md` say the same of "becomes blocked", by the answer above. `cargo xtask dsl check`
  enforces the gate's rule, with a negative test for an `or` holding a control-flow arm.
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

`docs/decisions/spelling.md` (shared material at a coordination); `docs/decisions/semantics.md` §4.1, §4.2, §4.6;
`plugins/builtin/primitives/event/or.syl`, `plugins/builtin/templates/action/establishFor.syl`,
`plugins/builtin/definitions/keyword_ability/bushido.syl`;
`plugins/canon/cards/{Aim High,Aether Tunnel,Arrest,Chub Toad}.syl`.
