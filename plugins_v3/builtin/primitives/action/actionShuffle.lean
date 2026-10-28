/-- Shuffle the specified library or face-down pile so no player knows its order [CR#701.24a]. -/
primitive actionShuffle (pile : Location) : Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.shuffle pile
}
