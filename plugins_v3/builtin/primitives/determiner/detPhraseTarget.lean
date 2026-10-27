primitive detPhraseTarget (quantity : Semantics.Quantity) : Semantics.DetPhrase := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.DetPhrase.target quantity
}
