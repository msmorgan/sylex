card unstableShapeshifter : Spelled :=
  { name := "Unstable Shapeshifter", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [shapeshifter],
    text :=
      [ whenever (does (a (and [isCreature, isOther])) enter)
          (establish (copyChange thisCreature (that creature) [exceptThisAbility])) ],
    power := stat 0, toughness := stat 1 }
