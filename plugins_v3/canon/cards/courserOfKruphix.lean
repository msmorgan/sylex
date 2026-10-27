card courserOfKruphix : Spelled :=
  { name := "Courser of Kruphix", cost := [generic 1, pip .green, pip .green],
    types := [.enchantment, .creature], subtypes := [centaur],
    text :=
      [ static (visibility .reveal you topOfLibrary),
        static playLandsFromTop,
        whenever (does (a (and [isLand, youControl])) enter)
          (gainLife 1) ],
    power := stat 2, toughness := stat 4 }
