/-- "<subject> becomes blocking <blocked>" -/
template becomeBlocking (subject blocked : NounPhrase) : Action := {
  spelling := "<subject> becomes blocking <blocked>",
  body :=
    combat subject (combatUpdateBlocking Semantics.AttachMove.attached blocked)
}
