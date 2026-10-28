/-- "with toughness <cmp> <amt>" -/
template hasToughness (cmp : Comparator) (amt : Amount) : Predicate := {
  spelling := "with toughness <cmp> <amt>",
  body :=
    compare cmp amt [.stat .toughness]
}
