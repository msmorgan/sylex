/-- "with toughness <cmp> <amt>" -/
template hasToughness (cmp : Comparator) (amt : Amount) : Predicate := {
  spelling := "with toughness <cmp> <amt>",
  body :=
    Semantics.Templates.Predicate.predicateCompare [Semantics.ProjAxis.stat Semantics.Stat.toughness] cmp amt
}
