/-- "with mana value <cmp> <amt>" -/
template hasManaValue (cmp : Comparator) (amt : Amount) : Predicate := {
  spelling := "with mana value <cmp> <amt>",
  body :=
    predicateCompare [Semantics.ProjAxis.stat Semantics.Stat.manaValue] cmp amt
}
