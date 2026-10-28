card := {
  name := "Patrician Geist",
  cost := [2, blue],
  types := [creature],
  subtypes := [spirit, knight],
  text := [
    flying,
    getsPt (allOf [spirit, youControl, isOther]) (up 1) (up 1),
    costShift (allOf [isSpell, did you (cast which (from_ := yourGraveyard)) thisObject]) (less 1)
  ],
  power := 2,
  toughness := 2
}
