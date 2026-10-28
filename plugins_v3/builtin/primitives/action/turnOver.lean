/-- Turn the subject over, optionally specifying its resulting characteristics. -/
primitive turnOver (subject : NounPhrase) (into : Option Predicate := none) : Action := {
  body := Semantics.Primitives.Action.turnOver subject into
}
