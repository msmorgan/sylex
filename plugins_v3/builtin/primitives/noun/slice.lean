/-- The specified number of cards from one end of a pile. -/
primitive slice (end_ : PileEnd) (amount : Amount) (pile : Location) : NounPhrase := {
  body :=
    Semantics.Primitives.NounPhrase.slice end_ amount pile
}
