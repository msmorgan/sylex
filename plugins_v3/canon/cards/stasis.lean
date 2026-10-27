card stasis : Spelled :=
  { name := "Stasis", cost := [generic 1, pip .blue], types := [.enchantment],
    text :=
      [ static (partSkip (bare isPlayer) .untapStep),
        at_ (beginningOfYour .upkeep)
          (doUnless (sacrifice thisEnchantment) [pip .blue]) ] }
