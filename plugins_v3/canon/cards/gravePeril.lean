/-- Grave Peril -/
card gravePeril : Spelled :=
  { name := "Grave Peril", cost := [generic 1, pip .black], types := [.enchantment],
    text :=
      [ when (does (a (and [isCreature, non (hasColor .black)])) enter)
          (doIfDone (sacrifice thisEnchantment) (destroy (that (.type
              .creature)))) ] }
