/-- "<subject> becomes blocking <blocked>" -/
template becomeBlocking (subject blocked : NounPhrase) : Action := {
  spelling := "<subject> becomes blocking <blocked>",
  body :=
    Semantics.Templates.Action.combat subject (Semantics.Templates.CombatUpdate.combatUpdateBlocking Semantics.AttachMove.attached blocked)
}
