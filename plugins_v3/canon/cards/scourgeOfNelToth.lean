/-- Scourge of Nel Toth -/
card := {
  name := "Scourge of Nel Toth",
  cost := [5, black, black],
  types := [creature],
  subtypes := [zombie, dragon],
  text := [
    flying,
    can you
      (cast this_ (from_ := yourGraveyard)
         (payment := payingInstead
            (compound [[black, black], (sacrifice (counted (exactly 2) isCreature))])))
  ],
  power := 6,
  toughness := 6
}
