primitive exchangedValues (left : Semantics.Amount) (right : Semantics.Amount) : Semantics.Exchanged := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Exchanged.values left right
}
