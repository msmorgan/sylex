card bloodfireEnforcers : Spelled :=
  { name := "Bloodfire Enforcers", cost := [generic 3, pip .red], types := [.creature],
    subtypes := [human, monk],
    text :=
      [ static (onlyWhile
          (conjunction [abilityGrant thisCreature firstStrike, abilityGrant it
              trample])
          (and
            [ exists_ (and [isInstant, in_ yourGraveyard]),
              exists_ (and [isSorcery, in_ yourGraveyard]) ])) ],
    power := stat 5, toughness := stat 2 }
