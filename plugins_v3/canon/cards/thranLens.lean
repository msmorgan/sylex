/-- Thran Lens -/
card thranLens : Spelled :=
  { name := "Thran Lens", cost := [generic 2], types := [.artifact],
    text := [static (qualityChange (allOf isPermanent) .sets (colored (.some [])))] }
