card vernalEquinox : Spelled :=
  { name := "Vernal Equinox", cost := [generic 3, pip .green], types := [.enchantment],
    text :=
      [ static
          (can (a isPlayer) (cast (bare (and [isSpell, or [isCreature, isEnchantment]])))
             (asThough := asThoughHadFlash them)) ] }
