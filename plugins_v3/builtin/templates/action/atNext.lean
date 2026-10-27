template atNext (part : TurnPart) (action : Action) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Templates.Action.delay (Semantics.Templates.GameEvent.beginningOf part Option.none (Semantics.Templates.TurnRef.nextTime Option.none)) action
}
