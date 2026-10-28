/-- Radiant Flames (converge is an ability word [CR#207.2c]) -/
card := {
  name := "Radiant Flames",
  cost := [2, red],
  types := [sorcery],
  text := [spell (instruct this_ (dealDamage (colorsSpentToCast this_) (each isCreature)))]
}
