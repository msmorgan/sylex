template chooseOne (first second : Action) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Templates.Action.actionChooseModes (Semantics.Templates.Quantity.exactly 1)
      [(Option.none, Option.none, first), (Option.none, Option.none, second)]
}
