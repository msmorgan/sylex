/-- "it" -/
primitive it : NounPhrase := {
  spelling := "it",
  body :=
    Semantics.Primitives.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one
}
