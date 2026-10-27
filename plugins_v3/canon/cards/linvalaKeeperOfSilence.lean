card linvalaKeeperOfSilence : Spelled :=
  { name := "Linvala, Keeper of Silence", cost := [generic 2, pip .white, pip .white],
    supertypes := [.legendary], types := [.creature], subtypes := [angel],
    text :=
      [ flying,
        static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                         abilityOf (allOf (and [isCreature, opponentsControl])) ])))) ],
    power := stat 3, toughness := stat 4 }
