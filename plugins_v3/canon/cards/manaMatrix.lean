card manaMatrix : Spelled :=
  { name := "Mana Matrix", cost := [generic 6], types := [.artifact],
    text :=
      [ static ((costShift (allOf (and [or [isInstant, isEnchantment], isSpell,
                  did you (cast which) thisObject])) (less 2))) ] }
