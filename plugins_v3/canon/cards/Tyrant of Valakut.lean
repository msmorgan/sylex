/-- Tyrant of Valakut -/
card := {
  name := "Tyrant of Valakut",
  cost := [5, red, red],
  types := [creature],
  subtypes := [dragon],
  text := [
    surge [3, red, red],
    flying,
    when (does thisCreature enter) (if_ := keywordCostPaid surge thisCreature)
      (instruct thisCreature (dealDamage 3 (target isAnyTarget)))
  ],
  power := 5,
  toughness := 4
}
