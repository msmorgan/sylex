/-- Fuming Effigy -/
card := {
  name := "Fuming Effigy",
  cost := [3, red],
  types := [creature],
  subtypes := [spirit],
  text := [
    whenever (does (counted (orMore 1) (cardIn yourGraveyard)) (leave (from_ := yourGraveyard)))
      (instruct this_ (dealDamage 1 eachOpponent))
  ],
  power := 4,
  toughness := 3
}
