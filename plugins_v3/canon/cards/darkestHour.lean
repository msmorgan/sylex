/-- Darkest Hour -/
card darkestHour : Spelled :=
  { name := "Darkest Hour", cost := [pip .black], types := [.enchantment],
    text := [static (qualityChange (allOf isCreature) .sets (colored (.some [.black])))] }
