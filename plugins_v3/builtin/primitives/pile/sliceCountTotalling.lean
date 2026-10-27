primitive sliceCountTotalling (axis : Semantics.ProjAxis) (comparator : Semantics.Comparator) (bound : Semantics.Amount) : Semantics.SliceCount := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.SliceCount.totalling axis comparator bound
}
