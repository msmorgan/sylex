/-- Aya of Alexandria -/
card ayaOfAlexandria : Spelled :=
  { name := "Aya of Alexandria", cost := [generic 2, pip .red, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, assassin],
    text :=
      [ menace, lifelink,
        whenever
          (does (a (and [isHistoric, isCreature, youControl]))
             (dealDamage (to := a isPlayer) (kind := some .combat)))
          (create 1 (written
            { characteristics :=
              { colors := [.black], types := [.creature], subtypes := [assassin],
                text := [menace], power := stat 1, toughness := stat 1 } })) ],
    power := stat 4, toughness := stat 3 }
