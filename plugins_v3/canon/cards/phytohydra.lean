card phytohydra : Spelled :=
  { name := "Phytohydra", cost := [generic 2, pip .green, pip .white, pip .white],
    types := [.creature], subtypes := [plant, hydra],
    text :=
      [ static (replace (would any (dealDamage anyAmount thisCreature)) (instead (putCounters thatMuch p1p1Counter it))) ],
    power := stat 1, toughness := stat 1 }
