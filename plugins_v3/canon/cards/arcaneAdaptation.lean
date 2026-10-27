card arcaneAdaptation : Spelled :=
  { name := "Arcane Adaptation", cost := [generic 2, pip .blue], types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment (.subtype .creature)),
        static (qualityChange (and [
            allOf (and [isCreature, youControl]),
            allOf (and [isCreature, isSpell, youControl]),
            allOf (and [isCard, isCreature, youOwn, not (in_ battlefield)]) ]) .adds
            (chosenQuality (ofChosen (.subtype .creature)))) ] }
