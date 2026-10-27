/-- Convincing Mirage -/
card convincingMirage : Spelled :=
  { name := "Convincing Mirage", cost := [generic 1, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isLand,
        static (entersChoosingFrom thisAura (.subtype .land) basicTypesOnly),
        static (qualityChange (enchanted land) .sets
          (chosenQuality (ofChosen (.subtype .land)))) ] }
