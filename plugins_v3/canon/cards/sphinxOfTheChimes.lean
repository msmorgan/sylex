card := {
  name := "Sphinx of the Chimes",
  cost := [4, blue, blue],
  types := [creature],
  subtypes := [sphinx],
  text := [
    flying,
    activated (perform (discard (withTheSameName (counted (exactly 2) [non isLand, in_ yourHand]))))
      (draw 4)
  ],
  power := 5,
  toughness := 6
}
