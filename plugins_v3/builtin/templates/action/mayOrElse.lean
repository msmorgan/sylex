template mayOrElse (body otherwise : Action) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Templates.Action.may body Option.none (Option.some otherwise)
}
