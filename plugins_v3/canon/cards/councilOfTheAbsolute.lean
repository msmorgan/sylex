/-- Council of the Absolute -/
card councilOfTheAbsolute : Spelled :=
  { name := "Council of the Absolute", cost := [generic 2, pip .white, pip .blue],
    types := [.creature], subtypes := [human, advisor],
    text :=
      [ static (entersChoosingFrom thisCreature .cardName
          (nameOfCard (not (or [isCreature, isLand])))),
        static (cant yourOpponents (cast (bare (and [isSpell, named chosenName])))),
        static (costShift (allOf (and [isSpell, named chosenName, did you (cast which) thisObject]))
          (less 2)) ],
    power := stat 2, toughness := stat 4 }
