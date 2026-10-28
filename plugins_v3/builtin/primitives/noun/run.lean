/-- The cards from one end of a pile through the stopping predicate. -/
primitive run (end_ : PileEnd) (stop : Predicate) (pile : Location) : NounPhrase := {
  body :=
    Semantics.Primitives.NounPhrase.run end_ stop pile
}
