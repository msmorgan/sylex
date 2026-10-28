/-- Bard, Heir of Girion -/
card := {
  name := "Bard, Heir of Girion",
  cost := [2, white, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, archer],
  text := [
    reach,
    vigilance,
    getsPt (allOf [isCreature, youControl, isOther]) (up 1) (up 1),
    whenever (does you attack) (draw 1)
  ],
  power := 4,
  toughness := 4
}
