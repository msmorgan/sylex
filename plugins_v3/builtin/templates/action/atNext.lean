template atNext (part : TurnPart) (action : Action) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    delay (beginningOf part none (nextTime none)) action
}
