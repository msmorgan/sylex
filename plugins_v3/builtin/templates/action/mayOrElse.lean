/-- "You may <body>. If you don't, <otherwise>.": an optional Action with only a failure branch. -/
template mayOrElse (body otherwise : Action) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    may body none (some otherwise)
}
