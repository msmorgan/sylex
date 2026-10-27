/-- "itself" / "himself" / "herself" -/
primitive itself : NounPhrase := {
  spelling := "itself",
  body :=
    Semantics.Primitives.NounPhrase.refl Semantics.Plurality.one
}
