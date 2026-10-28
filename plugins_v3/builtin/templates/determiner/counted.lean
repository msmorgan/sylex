/-- "N …", "up to N …" -/
template counted (q : Quantity) (p : Predicate) : NounPhrase := {
  spelling := "<q> <p>",
  body :=
    described (count q none) p
}
