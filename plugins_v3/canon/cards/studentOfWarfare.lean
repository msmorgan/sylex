/-- Student of Warfare -/
card studentOfWarfare : Spelled := { body := .leveler
  { characteristics :=
    { name := "Student of Warfare", cost := [pip .white], types := [.creature],
      subtypes := [human, knight],
      text := [levelUp [pip .white]], power := stat 1, toughness := stat 1 } }
  [ levelBand (.between 2 6) 3 3 [firstStrike],
    levelBand (.plus 7) 4 4 [doubleStrike] ] }
