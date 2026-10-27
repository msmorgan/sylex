primitive moveRiderNot (rider : Semantics.MoveRider) : Semantics.MoveRider := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.MoveRider.not rider
}
