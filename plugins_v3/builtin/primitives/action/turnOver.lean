primitive turnOver (subject : NounPhrase) (into : Option Predicate := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body := Semantics.Primitives.Action.turnOver subject into
}
