/-- "each object" -/
template everyObject : NounPhrase := {
  spelling := "each object",
  body :=
    Semantics.Templates.NounPhrase.allOf (Semantics.Templates.Predicate.and [])
}
