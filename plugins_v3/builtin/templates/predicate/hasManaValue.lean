/-- "with mana value <cmp> <amt>" -/
template hasManaValue (cmp : Comparator) (amt : Amount) : Predicate := {
  spelling := "with mana value <cmp> <amt>",
  body :=
    Semantics.Templates.Predicate.predicateCompare [Semantics.ProjAxis.stat Semantics.Stat.manaValue] cmp amt
}
