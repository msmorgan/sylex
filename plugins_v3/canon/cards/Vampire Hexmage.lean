/-- Vampire Hexmage -/
card := {
  name := "Vampire Hexmage",
  cost := [black, black],
  types := [creature],
  subtypes := [vampire, shaman],
  text := [firstStrike, activated (sacrifice thisCreature) (removeAllCounters none (target isPermanent))],
  power := 2,
  toughness := 1
}
