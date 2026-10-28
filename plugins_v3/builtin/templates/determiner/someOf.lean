/-- "N of <group>" -/
template someOf (quantity : Quantity) (group : NounPhrase) : NounPhrase := {
  spelling := "<quantity> of <group>",
  body :=
    nounPhraseSomeOf (counted quantity) none group
}
