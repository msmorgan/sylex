card giantCindermaw : Spelled :=
  { name := "Giant Cindermaw", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [dinosaur, beast],
    text := [trample, static (cant (bare isPlayer) (gainLife anyAmount))],
    power := stat 4, toughness := stat 3 }
