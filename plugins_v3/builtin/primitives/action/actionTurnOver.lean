primitive actionTurnOver (subject : Semantics.NounPhrase) (into : Option Semantics.Predicate) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.turnOver subject into
}
