primitive target (quantity : Quantity) : DetPhrase := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.DetPhrase.target quantity
}
