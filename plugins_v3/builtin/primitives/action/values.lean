primitive values (left : Amount) (right : Amount) : Exchanged := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Exchanged.values left right
}
