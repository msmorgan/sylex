/-- "<quantity> of <group>": a counted slice of a named group. -/
primitive nounPhraseSomeOf (count : SliceCount) (description : Option Predicate) (group : NounPhrase) : NounPhrase := {
  body := Semantics.Primitives.NounPhrase.someOf count description group
}
