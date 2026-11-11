---
needs: [cleanup-step6-surface]
---

# Four canon terms that do not say what their cards print

## Why

Each of these terms writes words its card does not print, or cannot print what it does:

- *Answered Prayers* prints "Whenever a creature you control enters, you gain 1 life." and writes
  `when(does(actor(a(and([isCreature, youControl]))), enter), …)`, the trigger word "When".
- *Helm of Possession* prints "Gain control of target creature for as long as you control this
  artifact and this artifact remains tapped." and writes
  `forAsLongAs(and([is(thisArtifact, youControl), is(thisArtifact, isTapped)]))`: the first part
  composes "this artifact is you control", and the second prints "is" where the card prints
  "remains". No Condition declaration spells "$player controls $object".
- *Juxtapose* prints "You and target player exchange control of the creature you each control
  with the greatest mana value." and writes `controlledBy(target(isPlayer))` twice in its first
  exchange. A copy of a phrase that introduces a Referent introduces it again
  (`docs/decisions/spelling.md`, on shared material at a coordination), so the term has two targets where the card has one.
- `isAnyTarget` (`plugins/builtin/templates/predicate/isAnyTarget.syl`) has no spelling, and its
  callers wrap it in a determiner: 35 calls in 32 canon files write `target(isAnyTarget)`, which
  `target`'s wording "target $p" would print as "target …", where the cards print "any target"
  [CR#115.4]; *Fall of the Titans* writes `targets(upTo(2), isAnyTarget)` for "up to two
  targets", with no type word to print after "target", and *Chandra's Pyrohelix* and *Electrolyze*
  write `targets(oneThrough(2), isAnyTarget)` for "one or two targets". `isAnyOtherTarget` (*Screaming Nemesis*,
  "any other target") is the same.

## Open questions

- **How *Juxtapose* writes "target player" once.** "The creature you each control" distributes
  over the coordinated subject "You and target player", and no declaration writes a relative
  clause whose "each" ranges over its clause's coordinated subject. Writing the second mention
  as `that(player)` is an anaphor the card does not print. Recommendation: the exchange's subject
  is written once as the coordination of `you` and `target(…)`, with a new Template for "the
  $nominal you each control", added with *Juxtapose* as its caller; the owner names it.
- **How "any target" is written.** Recommendation: one NounPhrase declaration spelled "any target"
  and one spelled "any other target", and a counted form for "up to two targets" and "one or two
  targets", each over the body `isAnyTarget` has today, so the callers write no determiner.

## Done when

- *Answered Prayers* writes `whenever`.
- *Helm of Possession*'s duration is written as printed, with the declarations it needs, each
  checked against the card's Oracle text.
- *Juxtapose*'s term holds one `target(…)` for its one "target player".
- No call in `plugins/` wraps `isAnyTarget` or `isAnyOtherTarget` in a determiner; every caller
  writes the form its card prints.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- *Answered Prayers*'s "it becomes", written `become(thisEnchantment, …)`:
  `syl-printed-it-self-references`, and its raw record: `syl-become-replaces-types`.

## Pointers

`plugins/canon/cards/{Answered Prayers,Helm of Possession,Juxtapose,Harsh Sustenance,Fall of the
Titans,Screaming Nemesis}.syl`; `plugins/builtin/templates/predicate/{isAnyTarget,isAnyOtherTarget}.syl`,
`plugins/builtin/templates/determiner/{target,targets}.syl`,
`plugins/builtin/primitives/timing/forAsLongAs.syl`; `docs/decisions/spelling.md` (shared material at a coordination).
