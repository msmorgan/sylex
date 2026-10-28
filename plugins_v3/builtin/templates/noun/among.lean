/-- "among <group>": the whole of a group, sliced. -/
template among (group : NounPhrase) : NounPhrase := {
  spelling := "among <group>",
  body :=
    nounPhraseSomeOf whole none group
}
