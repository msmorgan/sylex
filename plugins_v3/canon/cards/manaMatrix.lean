card manaMatrix : Spelled :=
  { name := "Mana Matrix", cost := [generic 6], types := [.artifact],
    text :=
      [ static (spellsCost (and [or [isInstant, isEnchantment], isSpell,
          did you (cast which) thisObject]) (less 2)) ] }
