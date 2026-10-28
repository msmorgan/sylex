template mayOrElse (body otherwise : Action) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    may body none (some otherwise)
}
