/-- Blood Sun -/
card bloodSun : Spelled :=
  { name := "Blood Sun", cost := [generic 2, pip .red], types := [.enchantment],
    text :=
      [ when (does thisEnchantment enter) (draw 1),
        static (allAbilityLoss (allOf isLand) isManaAbility) ] }
