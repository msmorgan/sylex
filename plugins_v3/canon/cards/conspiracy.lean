card conspiracy : Spelled :=
  { name := "Conspiracy", cost := [generic 3, pip .black, pip .black],
    types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment (.subtype .creature)),
        static (qualityChange (and [
            allOf (and [isCreature, youControl]),
            allOf (and [isCreature, isSpell, youControl]),
            allOf (and [isCard, isCreature, youOwn, not (in_ battlefield)]) ]) .sets
            (chosenQuality (ofChosen (.subtype .creature)))) ] }
