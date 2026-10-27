/-- Brimstone Mage -/
card brimstoneMage : Spelled := { body := .leveler
  { characteristics :=
    { name := "Brimstone Mage", cost := [generic 2, pip .red], types := [.creature],
      subtypes := [human, shaman],
      text := [levelUp [generic 3, pip .red]], power := stat 2, toughness := stat 2 } }
  [ levelBand (.between 1 2) 2 3
      [activated tapSymbol (instruct thisCreature (dealDamage 1 (target isAnyTarget)))],
    levelBand (.plus 3) 2 4
      [activated tapSymbol (instruct thisCreature (dealDamage 3 (target isAnyTarget)))] ] }
