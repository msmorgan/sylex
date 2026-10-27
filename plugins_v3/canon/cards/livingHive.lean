card livingHive : Spelled :=
  { name := "Living Hive", cost := [generic 6, pip .green, pip .green], types := [.creature],
    subtypes := [elemental, insect],
    text :=
      [ trample,
        whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
          (create thatMuch (creatureToken 1 1 [.green] [insect])) ],
    power := stat 6, toughness := stat 6 }
