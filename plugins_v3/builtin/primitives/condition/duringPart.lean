/-- The game is during the specified turn part. -/
primitive duringPart (part : TurnPart) (whose : Option NounPhrase := none) :
    Condition := {
  body :=
    Semantics.Primitives.Condition.duringPart part whose
}
