/-- "Exile <subject> until <event>." -/
template exileUntil (subject : NounPhrase) (event : GameEvent) : Action := {
  spelling := "exile <subject> until <event>",
  body :=
    Semantics.Templates.Action.holdUntil (Semantics.Definitions.exile subject) event
}
