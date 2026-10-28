/-- The actor skips the specified number of turn parts. -/
primitive skipPart (part : TurnPart) (count : Amount) : Action := {
  body :=
    Semantics.Primitives.Action.skipPart part count
}
