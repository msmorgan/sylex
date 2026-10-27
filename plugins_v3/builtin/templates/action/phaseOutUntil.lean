/-- "<subject> phases out until <event>" -/
template phaseOutUntil (subject : NounPhrase) (event : GameEvent) : Action := {
  spelling := "<subject> phases out until <event>",
  body :=
    Semantics.Templates.Action.holdUntil (Semantics.Templates.Action.setStatus Semantics.Status.phasedOut subject) event
}
