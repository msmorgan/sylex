/-- "with power <cmp> <amt>" -/
template hasPower (cmp : Comparator) (amt : Amount) : Predicate := {
  spelling := "with power <cmp> <amt>",
  body :=
    predicateCompare [.stat .power] cmp amt
}
