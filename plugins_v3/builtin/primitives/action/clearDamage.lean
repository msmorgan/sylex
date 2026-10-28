/-- Remove marked damage from the subject. -/
primitive clearDamage (subject : NounPhrase) : Action := {
  spelling := "remove all damage marked on <subject>",
  body :=
    Semantics.Primitives.Action.clearDamage subject
}
