primitive actionShuffle (pile : Location) : Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.shuffle pile
}
