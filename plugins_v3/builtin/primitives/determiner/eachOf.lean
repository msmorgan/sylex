/-- Each member of the named group. -/
primitive eachOf (group : NounPhrase) : NounPhrase := {
  spelling := "each of <group>",
  body :=
    Semantics.Primitives.NounPhrase.eachOf group
}
