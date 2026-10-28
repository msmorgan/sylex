card := {
  name := "Jace Beleren",
  cost := [1, blue, blue],
  supertypes := [legendary],
  types := [planeswalker],
  subtypes := [jace],
  text := [
    activated (loyaltySymbol (up 2)) (instruct eachPlayer (draw 1)),
    activated (loyaltySymbol (down 1)) (instruct (target isPlayer) (draw 1)),
    activated (loyaltySymbol (down 10)) (instruct (target isPlayer) (mill 20))
  ],
  loyalty := 3
}
