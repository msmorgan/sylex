card := {
  name := "Shelkin Brownie",
  cost := [1, green],
  types := [creature],
  subtypes := [ouphe],
  text := [
    activated (compound [tapSymbol])
      (establishThroughout
         (abilityLoss (target isCreature) [abilities (anyIn ⟨bandsWithOther, none⟩)]) untilEndOfTurn)
  ],
  power := 1,
  toughness := 1
}
