/-- "<subject> phases out until <event>" -/
template phaseOutUntil (subject : NounPhrase) (event : GameEvent) : Action := {
  spelling := "<subject> phases out until <event>",
  body :=
    holdUntil (setStatus Semantics.Status.phasedOut subject) event
}
