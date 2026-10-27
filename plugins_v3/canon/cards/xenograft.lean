card xenograft : Spelled :=
  { name := "Xenograft", cost := [generic 4, pip .blue], types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment (.subtype .creature)),
        static (qualityChange (allOf (and [isCreature, youControl])) .adds
          (chosenQuality (ofChosen (.subtype .creature)))) ] }
