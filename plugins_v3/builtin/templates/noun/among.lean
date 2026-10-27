/-- "among <group>": the whole of a group, sliced. -/
template among (group : NounPhrase) : NounPhrase := {
  spelling := "among <group>",
  body :=
    Semantics.Templates.NounPhrase.nounPhraseSomeOf Semantics.Templates.SliceCount.whole Option.none group
}
