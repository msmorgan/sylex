primitive limited (action : Action) (limit : UsageLimit) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.limited action limit
}
