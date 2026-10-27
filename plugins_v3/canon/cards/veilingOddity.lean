/-- Veiling Oddity -/
card veilingOddity : Spelled :=
  { name := "Veiling Oddity", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [illusion],
    text := [suspend 4 [generic 1, pip .blue], veilingOddityLine],
    power := stat 2, toughness := stat 3 }
