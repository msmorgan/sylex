primitive addTurnPart (part : TurnPart) (count : Amount)
    (anchor : Option TurnPart := none) (followedBy : Option TurnPart := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.addTurnPart part anchor count followedBy
}
