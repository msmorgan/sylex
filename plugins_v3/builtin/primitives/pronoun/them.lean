/-- "them" -/
primitive them : NounPhrase := {
  spelling := "them",
  body :=
    Semantics.Primitives.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.many
}
