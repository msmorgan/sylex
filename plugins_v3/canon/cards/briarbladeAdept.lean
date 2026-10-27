/-- Briarblade Adept -/
card briarbladeAdept : Spelled :=
  { name := "Briarblade Adept", cost := [generic 4, pip .black], types := [.creature],
    subtypes := [elf, assassin],
    text :=
      [ whenever (does thisCreature attack)
          (get (target (and [isCreature, opponentsControl])) (down 1) (down 1) untilEndOfTurn),
        encore [generic 3, pip .black] ],
    power := stat 3, toughness := stat 4 }
