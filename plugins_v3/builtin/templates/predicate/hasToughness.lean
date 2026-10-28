/-- "with toughness <cmp> <amt>" -/
template hasToughness (cmp : Comparator) (amt : Amount) : Predicate := {
  spelling := "with toughness <cmp> <amt>",
  body :=
    predicateCompare [Semantics.ProjAxis.stat Semantics.Stat.toughness] cmp amt
}
