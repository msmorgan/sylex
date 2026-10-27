primitive sliceCountCounted (quantity : Semantics.Quantity) : Semantics.SliceCount := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.SliceCount.counted quantity
}
