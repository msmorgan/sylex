/-- "with mana value <cmp> <amt>" -/
template hasManaValue (cmp : Comparator) (amt : Amount) : Predicate := {
  spelling := "with mana value <cmp> <amt>",
  body :=
    predicateCompare [.stat .manaValue] cmp amt
}
