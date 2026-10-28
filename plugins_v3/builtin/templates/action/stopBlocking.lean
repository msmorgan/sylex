/-- "<subject> stops blocking <blocked>" -/
template stopBlocking (subject blocked : NounPhrase) : Action := {
  spelling := "<subject> stops blocking <blocked>",
  body :=
    combat subject (combatUpdateBlocking Semantics.AttachMove.unattached blocked)
}
