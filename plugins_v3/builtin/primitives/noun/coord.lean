/-- Full-arity binding for `NounPhrase.coord`. -/
primitive coord (op : Coordinator) (phrases : List NounPhrase) : NounPhrase := {
  body := Semantics.Primitives.NounPhrase.coord op phrases
}
