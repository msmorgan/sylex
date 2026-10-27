/-- "other than <np>" -/
primitive otherThan (np : NounPhrase) : Predicate := {
  spelling := "other than <np>",
  body :=
    Semantics.Primitives.Predicate.otherThan np
}
