card phyrexianRevoker : Spelled :=
  { name := "Phyrexian Revoker", cost := [generic 2], types := [.artifact, .creature],
    subtypes := [phyrexian, horror],
    text :=
      [ static (entersChoosingFrom thisCreature .cardName (nameOfCard (non isLand))),
        static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                         abilityOf (allOf (and [isSource, named chosenName])) ])))) ],
    power := stat 2, toughness := stat 1 }
