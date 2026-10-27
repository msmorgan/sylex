/-- "<subject> stops blocking <blocked>" -/
template stopBlocking (subject blocked : NounPhrase) : Action := {
  spelling := "<subject> stops blocking <blocked>",
  body :=
    Semantics.Templates.Action.combat subject (Semantics.Templates.CombatUpdate.combatUpdateBlocking Semantics.AttachMove.unattached blocked)
}
