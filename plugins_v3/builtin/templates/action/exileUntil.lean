/-- "Exile <subject> until <event>." -/
template exileUntil (subject : NounPhrase) (event : GameEvent) : Action := {
  spelling := "exile <subject> until <event>",
  body :=
    .holdUntil (exile subject) event
}
