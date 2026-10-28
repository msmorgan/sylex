/-- The game is during the specified turn part. -/
primitive duringPart (part : TurnPart) (whose : Option NounPhrase := none) :
    Condition := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Condition.duringPart part whose
}
