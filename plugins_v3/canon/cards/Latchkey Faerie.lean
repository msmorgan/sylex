card := {
  name := "Latchkey Faerie",
  cost := [3, blue],
  types := [creature],
  subtypes := [faerie, rogue],
  text := [
    flying,
    prowl [2, blue],
    when (does thisCreature enter) (if_ := keywordCostPaid prowl thisCreature) (draw 1)
  ],
  power := 3,
  toughness := 1
}
