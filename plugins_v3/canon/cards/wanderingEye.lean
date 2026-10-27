card wanderingEye : Spelled :=
  { name := "Wandering Eye", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [illusion],
    text := [flying, static (visibility .reveal (bare isPlayer) wholeHand)],
    power := stat 1, toughness := stat 3 }
