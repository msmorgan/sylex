---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement, syl-single-face-card-call, cleanup-step6-surface]
---
# Spend purposes take noun phrases

## Why

`toCast(predicate: Predicate)` (`plugins/builtin/primitives/mana/toCast.syl`, "to cast
<predicate> spells") cannot take an anaphor, so "to cast that spell" and "to cast those spells"
cannot be written. Its three canon sites also print "spells" twice: *Ancient Ziggurat* and
*Mishra's Workshop* pass `and([…, isSpell])` to a spelling that already ends in "spells", and so
does the Template `spendOnlyToCastAnd`, which *Cavern of Souls* calls with `and([isCreature,
isSpell, …])`.

The other two spend purposes that take a `Predicate` print a noun phrase too:

- `toTurnFaceUp(subject: Predicate)`, "to turn <subject> face up". Its one caller, *Overgrown
  Zealot*, prints "Spend this mana only to turn permanents face up" and writes
  `toTurnFaceUp(isPermanent)`, which spells "to turn permanent face up". It is the only Oracle
  spend restriction that turns something face up.
- `toActivate(source: Option(Predicate))`, "to activate abilities [of <source>]", with no
  `spelling` and no caller in `plugins/`. Oracle prints a bare "to activate abilities"
  (*Boommobile*, *Pit Automaton*) and "of" with a plural noun phrase: "of creature sources"
  (*Shang-Chi, Master of Kung Fu*), "of land sources" (*Sunken Citadel*), "of artifact sources"
  (*Steelswarm Operator*), "of artifacts" (*Soldevi Machinist*).

## Design

- `toCast(spells: NounPhrase)`, so "to cast that spell" and "to cast those spells" name an
  antecedent; a generic plural ("to cast creature spells") is `bare(<description>)`.
- `toTurnFaceUp(subject: NounPhrase)` and `toActivate(source: Option(NounPhrase))` likewise;
  "permanents", "creature sources" and "artifacts" are bare plurals.

## Done when

- `toCast`, `toTurnFaceUp` and `toActivate` take a NounPhrase, and *Ancient Ziggurat*,
  *Mishra's Workshop*, *Unblinking Observer*, `spendOnlyToCastAnd` and *Overgrown Zealot* write
  it as printed, each description reading its printed words once (re-grep the three in
  `plugins/` when starting).
- `toActivate` is spelled with its optional "of $source", and *Soldevi Machinist* and *Sunken
  Citadel* are in `plugins/canon/cards/` calling it, checked against their Oracle text.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- A cast permission publishing "that spell" to the text after it (*Rogue Class*, *Summon: Esper
  Valigarmanda*): `syl-cast-permission-spell-anaphor`.
- Other activation purposes Oracle prints: "to activate an ability or cast an artifact spell"
  (*Automated Artificer*), "an ability of a creature" (*Tazri, Stalwart Survivor*'s granted
  ability), "power-up abilities" (*Quinjet Technician*).

## Pointers

- `plugins/builtin/primitives/mana/{toCast,toTurnFaceUp,toActivate,spendOnly}.syl`
- `plugins/builtin/templates/determiner/bare.syl`
- `plugins/canon/cards/Ancient Ziggurat.syl`, `Mishra's Workshop.syl`, `Unblinking Observer.syl`,
  `Cavern of Souls.syl`, `Overgrown Zealot.syl`
- `plugins/builtin/templates/mana/spendOnlyToCastAnd.syl`
