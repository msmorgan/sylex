card wizenedSnitches : Spelled :=
  { name := "Wizened Snitches", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [faerie, rogue],
    text := [flying, static (visibility .reveal (bare isPlayer) topOfLibrary)],
    power := stat 1, toughness := stat 3 }
