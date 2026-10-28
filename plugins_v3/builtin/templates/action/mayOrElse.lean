/-- "You may <body>. If you don't, <otherwise>.": an optional Action with only a failure branch. -/
template mayOrElse (body otherwise : Action) : Action := {
  spelling := "You may <body>. If you don't, <otherwise>.",
  body :=
    may body none (some otherwise)
}
