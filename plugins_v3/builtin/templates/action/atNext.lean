/-- "At the beginning of the next <part>, <action>": a delayed triggered ability [CR#603.7a]. -/
template atNext (part : TurnPart) (action : Action) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    delay (beginningOf part none (nextTime none)) action
}
