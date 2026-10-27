card cryptLurker : Spelled :=
  { name := "Crypt Lurker", cost := [generic 3, pip .black], types := [.creature],
    subtypes := [horror],
    text :=
      [ when (does thisCreature enter)
          (may
            (chooseOne (sacrifice (a isCreature)) (discard (a (and [isCreature, in_ yourHand]))))
            (draw 1)) ],
    power := stat 3, toughness := stat 4 }
