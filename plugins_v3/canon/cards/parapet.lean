/-- Parapet -/
card parapet : Spelled :=
  { name := "Parapet", cost := [generic 1, pip .white], types := [.enchantment],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        static (getsPt (allOf (and [isCreature, youControl])) (up 0) (up 1)) ] }
