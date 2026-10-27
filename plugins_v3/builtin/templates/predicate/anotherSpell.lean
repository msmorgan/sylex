/-- "another spell" -/
template anotherSpell : Predicate := {
  spelling := "another spell",
  body :=
    Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isSpell, Semantics.Templates.Predicate.isOther]
}
