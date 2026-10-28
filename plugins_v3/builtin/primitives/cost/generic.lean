primitive generic (change : Delta Amount) (floor : Option Amount) : CostShift := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CostShift.generic change floor
}
