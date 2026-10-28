primitive count (quantity : Quantity) (mode : Option ChoiceMode) : DetPhrase := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.DetPhrase.count quantity mode
}
