/-- "with power <cmp> <amt>" -/
template hasPower (cmp : Comparator) (amt : Amount) : Predicate := {
  spelling := "with power <cmp> <amt>",
  body :=
    Semantics.Templates.Predicate.predicateCompare [Semantics.ProjAxis.stat Semantics.Stat.power] cmp amt
}
