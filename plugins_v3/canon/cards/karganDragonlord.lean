/-- Kargan Dragonlord -/
card karganDragonlord : Spelled := { body := .leveler
  { characteristics :=
    { name := "Kargan Dragonlord", cost := [pip .red, pip .red], types := [.creature],
      subtypes := [human, warrior],
      text := [levelUp [pip .red]], power := stat 2, toughness := stat 2 } }
  [ levelBand (.between 4 7) 4 4 [flying],
    levelBand (.plus 8) 8 8
      [ flying, trample,
        activated [pip .red]
          (get thisCreature (up 1) (up 0) untilEndOfTurn) ] ] }
