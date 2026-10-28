card := {
  name := "Nyxathid",
  cost := [1, black, black],
  types := [creature],
  subtypes := [elemental],
  text := [
    entersChoosingPlayer thisCreature anOpponent,
    getsPt thisCreature (down (countOf (cardIn (handOf (the isChosenPlayer)))))
      (down (countOf (cardIn (handOf (the isChosenPlayer)))))
  ],
  power := 7,
  toughness := 7
}
