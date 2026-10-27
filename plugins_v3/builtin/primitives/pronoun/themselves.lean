/-- "themselves" -/
primitive themselves : NounPhrase := {
  spelling := "themselves",
  body :=
    Semantics.Primitives.NounPhrase.refl Semantics.Plurality.many
}
