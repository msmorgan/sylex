---
needs: []
---

# Read a premodifier over a whole coordination of nominal heads

## Why

Where a premodifier stands on the first of coordinated nominal heads, the grammar reads it over
that conjunct only, though the cards often mean it over the whole coordination. *Mox Amber*'s
ruling reads "legendary creatures and planeswalkers you control" as "legendary creatures and
legendary planeswalkers you control"; *Ghostly Flame*'s reads "Black and/or red permanents and
spells" as "red and black spells and permanents". *Justice*'s "a red creature or spell" has the
same two readings. In a survey of the in-scope coordinations with a leading premodifier, 65 of the
78 that parsed had only the narrow reading, 6 both, 7 neither, and 13 did not parse. A stacked
premodifier over a coordination has no reading at all: *Brimaz, Blight of Oreskos*'s "Phyrexian
creature or artifact creature spell" gets "Phyrexian [creature or artifact] creature spell" and
"[Phyrexian creature] or [artifact creature spell]", never "[Phyrexian creature or artifact
creature] spell". No recovery can choose a Reading the forest lacks, and the narrow Readings are
grammatical, so both belong in the forest (`english.md` §4, "Structure over suppression").

Reproduce with `cargo xtask english parse --limit 40 "<line>"` and read the `Premodifier` and
coordination nodes of every Reading:

- `"{T}: Add one mana of any color among legendary creatures and planeswalkers you control."`
  (*Mox Amber*): 11 Readings, "legendary" premodifies only "creatures" in each.
- `"Whenever a red creature or spell deals damage, this enchantment deals that much damage to
  that creature's or spell's controller."` (*Justice*): 1 Reading, `Premodifier` "red creature"
  under `Coordination:Nominal#Disjunctive` "red creature or spell".
- `"Black and/or red permanents and spells are colorless sources of damage."` (*Ghostly Flame*):
  3 Readings, "Black and/or red" premodifies only "permanents" in each.
- `"Whenever you cast a Phyrexian creature or artifact creature spell, incubate X, where X is
  that spell's mana value."` (*Brimaz, Blight of Oreskos*): 4 Readings, none with "Phyrexian
  creature or artifact creature" as one premodifier of "spell".

## Done when

- *Mox Amber*'s, *Justice*'s and *Ghostly Flame*'s lines each have a Reading with the
  premodifier over the whole coordination of heads, and keep their Reading with it over the first
  conjunct only, in grammar tests.
- *Brimaz, Blight of Oreskos*'s line has a Reading in which "Phyrexian creature or artifact
  creature" premodifies "spell", in a grammar test.
- The leading-premodifier sites of the survey are re-parsed, and the landing reports how many
  have both readings, with each that still lacks one explained.
- `cargo xtask english strata --check` and `cargo test -p sylex_english` pass.

## Not in scope

Which of the two Readings recovery takes for a card. Postmodifiers after coordinated heads ("an
artifact or creature you control"), which already get both readings.

## Pointers

- `crates/english/grammar/english.fug`: `Premodifier`, `CoordinationPremodifier` (an
  *and*-coordination of singular nominals only), `NominalPremodifier`, the `Nominal` and
  `NounPremodifier` coordinations.
- `docs/decisions/english.md` §4.
- Local evidence (gitignored): `docs/evidence/cleanup-step6-syl/coordination-attachment-research.md`
  (§3, §3.1 and §4 Q3, with the site list).
