---
needs: [english-category-restriction, english-pre-post-dependent-nesting]
---

# Read a premodifier over a whole coordination of nominal heads

## Why

Where a premodifier stands on the first of coordinated nominal heads, the grammar reads it over
that conjunct only, though the cards often mean it over the whole coordination. *Mox Amber*'s
ruling reads "legendary creatures and planeswalkers you control" as "legendary creatures and
legendary planeswalkers you control"; *Ghostly Flame*'s reads "Black and/or red permanents and
spells" as "red and black spells and permanents". *Justice*'s "a red creature or spell" has the
same two readings. Of the 252 leading-premodifier sites in the forest database (a `Nominal`
coordination whose first conjunct is a bare nominal under a `Premodifier`, on 243 units), 18 had
the wide reading as well. A stacked
premodifier over a coordination has no reading at all: *Brimaz, Blight of Oreskos*'s "Phyrexian
creature or artifact creature spell" gets "Phyrexian [creature or artifact] creature spell" and
"[Phyrexian creature] or [artifact creature spell]", never "[Phyrexian creature or artifact
creature] spell". No recovery can choose a Reading the forest lacks, and the narrow Readings are
grammatical, so both belong in the forest (`english.md` §4, "Structure over suppression").

Reproduce with `cargo xtask english forestwalk --as-card NAME '<line>'` and read the forks of
the premodifier and the coordination:

- `"{T}: Add one mana of any color among legendary creatures and planeswalkers you control."`
  (*Mox Amber*): 10 Readings, "legendary" premodifies only "creatures" in each.
- `"Whenever a red creature or spell deals damage, this enchantment deals that much damage to
  that creature's or spell's controller."` (*Justice*): 1 Reading, `Premodifier` "red creature"
  under `Coordination:Nominal#Disjunctive` "red creature or spell".
- `"Black and/or red permanents and spells are colorless sources of damage."` (*Ghostly Flame*):
  3 Readings, "Black and/or red" premodifies only "permanents" in each.
- `"Whenever you cast a Phyrexian creature or artifact creature spell, incubate X, where X is
  that spell's mana value."` (*Brimaz, Blight of Oreskos*): 2 Readings, none with "Phyrexian
  creature or artifact creature" as one premodifier of "spell".

## Design

- Generalize `CoordinationPremodifier` over the shared `Premodifier` category, keeping a measure
  before *counter* its kind as over one head (the one of `Premodifier`'s gates on its kind that
  changes a card here, *Dismantle*; a `Nominal` coordination with *counter* among its conjuncts
  resolves `KindLicense` to `Head`, *Invoke Justice*), and replace `head.Number = Plural` and
  `head.ConjunctNumber := Singular` (which admit only an *and*-coordination of singulars: every
  uncoordinated nominal values `ConjunctNumber` too) by a privative `NominalCoordination` that the
  `Nominal` coordinations value and a head passes up, so any coordinated `Nominal` (*Mox Amber*,
  *Justice*, *Ghostly Flame*), one with a Complement shared by Right Node Raising (*total [power
  and toughness] 5 or less*) included, admits both the wide-scope premodifier Reading (inside
  postmodifiers, `OuterModifier`) and the first-conjunct Reading.
- Drop `modifier.Polarity := Negative` on `adjunct NounPremodifier { host: NounPremodifierConjunct }`
  and give a `Nominal`-hosted Adjunct preposed without a comma `Fronting::Ambiguous`, as a
  Predicate one has, instead of `Fronting::Wide` in `sylex_syntax`, so *Brimaz*'s `"Phyrexian
  creature or artifact creature spell"`, `"instant or artifact creature spell"` and `"[legendary
  artifact] or enchantment card"` parse symmetrically, and the first conjunct's own premodifier
  is read narrow as well as wide (*[non-Lesson instant] and sorcery card*, Iroh, Grand Lotus).
  *Abuelo's Awakening*'s `"non-Aura enchantment"` keeps its Reading. An uncoordinated noun stack is
  unaffected: `NounPremodifierConjunct` is only a conjunct. No card has a coordination of noun
  premodifiers as such a conjunct's premodifier, so no gate refuses one (`english.md` §6).

## Done when

- *Mox Amber*'s, *Justice*'s and *Ghostly Flame*'s lines each have a Reading with the
  premodifier over the whole coordination of heads, and keep their Reading with it over the first
  conjunct only, in grammar tests.
- *Brimaz, Blight of Oreskos*'s line has a Reading in which "Phyrexian creature or artifact
  creature" premodifies "spell", and *Abuelo's Awakening* keeps its `"target artifact or non-Aura
  enchantment card"` Reading, in grammar tests.
- The leading-premodifier sites (as counted in Why) are re-parsed from the forest database, and
  the landing reports how many have both readings, with each that still lacks one explained.
- `cargo xtask english strata --check`, `cargo test -p sylex_syntax -p sylex_english` and
  `cargo clippy -p sylex_syntax -p sylex_english --tests -- -D warnings` pass.

## Not in scope

Which of the two Readings recovery takes for a card. Postmodifiers after coordinated heads ("an
artifact or creature you control"), which already get both readings.

## Pointers

- `crates/english/grammar/english.fug`: `Premodifier`, `CoordinationPremodifier`,
  `NominalPremodifier`, `NounPremodifierConjunct`, the `Nominal` and `NounPremodifier`
  coordinations.
- `crates/syntax/src/step.rs` (`Fronting`, `Summary::fronted`).
- `docs/design/english.md` §4.
- Local evidence (gitignored): `docs/evidence/cleanup-step6-syl/coordination-attachment-research.md`
  (§3, §3.1 and §4 Q3; its site list was in a session scratchpad and is gone).
