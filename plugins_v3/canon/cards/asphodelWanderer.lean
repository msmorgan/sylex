card asphodelWanderer : Spelled :=
  { name := "Asphodel Wanderer", cost := [pip .black], types := [.creature],
    subtypes := [skeleton, soldier],
    text := [activated [generic 2, pip .black] (regenerate thisCreature)],
    power := stat 1, toughness := stat 1 }
