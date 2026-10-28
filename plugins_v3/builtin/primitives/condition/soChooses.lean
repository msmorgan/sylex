primitive soChooses (chooser : NounPhrase) : Condition := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Condition.soChooses chooser
}
