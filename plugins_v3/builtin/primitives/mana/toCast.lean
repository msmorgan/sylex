/-- "to cast <predicate> spells" -/
primitive toCast (predicate : Predicate) : SpendPurpose := {
  spelling := "to cast <predicate> spells",
  body :=
    Semantics.Primitives.SpendPurpose.toCast predicate
}
