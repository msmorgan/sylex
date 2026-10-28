/-- "remove <subject> from combat" -/
template removeFromCombat (subject : NounPhrase) : Action := {
  spelling := "remove <subject> from combat",
  body :=
    combat subject (participation outsideCombat)
}
