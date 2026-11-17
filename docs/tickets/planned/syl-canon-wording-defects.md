---
needs: [cleanup-step6-surface]
---

# Canon terms that do not say what their cards print

## Why

Each of these terms writes words its card does not print, or cannot print what it does:

- *Answered Prayers* prints "Whenever a creature you control enters, you gain 1 life." and writes
  `when(does(actor(a(nominal(creature, post = [youControl]))), enter), …)`, the trigger word
  "When".
- *Helm of Possession* prints "Gain control of target creature for as long as you control this
  artifact and this artifact remains tapped." and writes
  `forAsLongAs(and([controls(you, this(artifact)), is(this(artifact), tapped)]))`, which prints "is"
  where the card prints "remains".
- *Juxtapose* prints "You and target player exchange control of the creature you each control
  with the greatest mana value." and writes `controlledBy(target(player))` twice in its first
  exchange. A copy of a phrase that introduces a Referent introduces it again
  (`docs/design/spelling.md`, on shared material at a coordination), so the term has two targets
  where the card has one.
- `anyTarget` (`plugins/builtin/templates/nominal/anyTarget.syl`) has no spelling, and its callers
  wrap it in a determiner: 39 calls in 36 canon files, 35 of them `target(anyTarget)`, which
  `target`'s wording "target $p" would print as "target …", where the cards print "any target"
  [CR#115.4]. *Fall of the Titans* writes `targets(upTo(2), anyTarget)` for "up to two targets",
  with no type word to print after "target"; *Chandra's Pyrohelix* and *Electrolyze* write
  `targets(oneOrTwo, anyTarget)` for "one or two targets"; and *Screaming Nemesis* writes
  `target(nominal(pre = [other], anyTarget))` for "any other target".
- A printed "and" that unions two types is written `or`, which prints "or": at `PreModifier` in
  *Arcane Melee* ("Instant and sorcery spells"), *Aura of Silence*, *Goblin Electromancer*, *Mana
  Matrix*, *Melek, Izzet Paragon*, *Precognition Field*, *Prismari, the Inspiration* and *Vernal
  Equinox*, and at `Nominal` in *Benevolent Blessing*, *Clarion Conqueror*, *Damping Matrix* and
  *Glaring Spotlight* ("spells and abilities"). No `PreModifier` coordination spells "and" with
  union meaning.
- *Lava Axe*, *Mogg Maniac* and *Searing Flesh* print "player or planeswalker" (or "opponent or
  planeswalker") and write the arms in the other order, `or([planeswalker, player])`.
- *Tide Shaper* and *Briarblade Adept* print "an opponent controls" and write `opponentsControl`.
- `allOf` stands where the text prints neither "all" nor a bare plural: `shadow`'s second clause
  ("a creature without shadow can't be blocked by creatures with shadow" [CR#702.28b], its
  attacker written `allOf(…)`), `equipment` and `fortification` ("can be attached to a creature",
  "to a land" [CR#301.5,301.6]), `theRingTemptsYou` ("Your Ring-bearer" [CR#701.54c]), the
  `yourOpponents` Template ("your opponents", over `allOf(opponent)`), and *Duel for Dominance*
  ("three or more creatures with different powers", written as a count of different powers among
  `allOf(…)`).
- Thirteen canon cards write the discarded card with an unprinted "in your hand":
  `discard(a(nominal(…, card, post = [in(yourHand)])))` in *Crypt Lurker*, *Abolish*, *Charnel
  Troll* and others, where the cards print "discard a creature card" and the like.
- `soulbond` writes its intervening condition as `is(this(creature), and([controlledBy(you), …]))`,
  "this creature is controlled by you", where [CR#702.95a] prints "if you control both this
  creature and another creature and both are unpaired". No plural anaphor spells "both", and no
  rule lets an anaphor in one conjunct of a condition read an indefinite of the conjunct before it.
- `copied` at `PreModifier` is called where *Rings of Brighthearth* and *Strionic Resonator* print
  "the copy", a noun phrase, and `choseExtreme` where *Menacing Ogre* prints "with the highest
  number" and "If you are one of those players".
- *Denry Klin, Editor in Chief* writes `this(creature)` twice in "put the same number of each
  kind of counter on that creature", a clause that prints no mention of Denry Klin: the counters
  are those of the condition before it, "if Denry Klin has counters on it".

## Open questions

- **How *Juxtapose* writes "target player" once.** "The creature you each control" distributes
  over the coordinated subject "You and target player", and no declaration writes a relative
  clause whose "each" ranges over its clause's coordinated subject. Writing the second mention
  as `that(player)` is an anaphor the card does not print. Recommendation: the exchange's subject
  is written once as the coordination of `you` and `target(…)`, with a new Template for "the
  $nominal you each control", added with *Juxtapose* as its caller; the owner names it.
- **How "any target" is written.** Recommendation: one NounPhrase declaration spelled "any target"
  and one spelled "any other target", and a counted form for "up to two targets" and "one or two
  targets", each over the body `anyTarget` has today, so the callers write no determiner.

## Done when

- *Answered Prayers* writes `whenever`.
- *Helm of Possession*'s duration prints "remains tapped", with the declarations it needs, each
  checked against the card's Oracle text.
- *Juxtapose*'s term holds one `target(…)` for its one "target player".
- No call in `plugins/` wraps `anyTarget` in a determiner; every caller writes the form its card
  prints.
- Each card listed under the union "and" prints "and" where its Oracle text does, with the
  declaration that needs, and any other canon card with the same defect is found from the parse
  and fixed with them.
- *Lava Axe*, *Mogg Maniac* and *Searing Flesh* write their coordination in printed order.
- *Tide Shaper* and *Briarblade Adept* print "an opponent controls".
- No `allOf` in `plugins/` stands for text that prints no "all": each site above writes the
  determiner its rule or card prints.
- No `discard` in canon writes a location its card does not print.
- `soulbond`'s conditions print [CR#702.95a]'s words, with the declarations and the accessibility
  rule that needs.
- *Rings of Brighthearth*, *Strionic Resonator* and *Menacing Ogre* write what they print, and
  `copied` and `choseExtreme` are gone.
- *Denry Klin, Editor in Chief*'s second ability writes no `this(…)` its text does not print.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- *Answered Prayers*'s raw `become` record: `syl-become-replaces-types`.

## Pointers

`plugins/canon/cards/{Answered Prayers,Helm of Possession,Juxtapose,Harsh Sustenance,Fall of the
Titans,Screaming Nemesis}.syl`; `plugins/builtin/templates/nominal/anyTarget.syl`,
`plugins/builtin/templates/determiner/{target,targets}.syl`,
`plugins/builtin/primitives/timing/forAsLongAs.syl`; `docs/design/spelling.md` (shared material at a coordination).
