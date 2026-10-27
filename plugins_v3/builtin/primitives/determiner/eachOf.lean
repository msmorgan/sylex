primitive eachOf (group : NounPhrase) : NounPhrase := {
  spelling := "each of <group>",
  body :=
    Semantics.Primitives.NounPhrase.eachOf group
}
