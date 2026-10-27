card grollub : Spelled :=
  { name := "Grollub", cost := [generic 2, pip .black], types := [.creature],
    subtypes := [beast],
    text := [whenever (does any (dealDamage (to := thisCreature))) (instruct eachOpponent (gainLife thatMuch))],
    power := stat 3, toughness := stat 3 }
