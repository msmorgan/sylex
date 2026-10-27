primitive detPhraseCount (quantity : Semantics.Quantity) (mode : Option Semantics.ChoiceMode) : Semantics.DetPhrase := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.DetPhrase.count quantity mode
}
