card divineIntervention : Spelled :=
  { name := "Divine Intervention", cost := [generic 6, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ static (entersWithCounters thisEnchantment 2 interventionCounter),
        at_ (beginningOfYour .upkeep)
          (removeCounters (exactly 1) interventionCounter thisEnchantment),
        when (does (by_ you) (removeCounters theLast interventionCounter thisEnchantment)) drawGame ] }
