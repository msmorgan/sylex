/-- Ainok Tracker -/
card ainokTracker : Spelled :=
  { name := "Ainok Tracker", cost := [generic 5, pip .red], types := [.creature],
    subtypes := [dog, scout],
    text := [firstStrike, morph [generic 4, pip .red]],
    power := stat 3, toughness := stat 3 }
