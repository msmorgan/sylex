primitive conditionSoChooses (chooser : Semantics.NounPhrase) : Semantics.Condition := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Condition.soChooses chooser
}
