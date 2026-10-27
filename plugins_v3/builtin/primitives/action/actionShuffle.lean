primitive actionShuffle (pile : Semantics.Location) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.shuffle pile
}
