/-- Frost Walker -/
card := {
  name := "Frost Walker",
  cost := [1, blue],
  types := [creature],
  subtypes := [elemental],
  text := [when (does (a (or [isSpell, abilityHead anyOnStack])) (target thisCreature)) (sacrifice it)],
  power := 4,
  toughness := 1
}
