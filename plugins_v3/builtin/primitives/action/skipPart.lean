/-- The actor skips the specified number of turn parts. -/
primitive skipPart (part : TurnPart) (count : Amount) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.skipPart part count
}
