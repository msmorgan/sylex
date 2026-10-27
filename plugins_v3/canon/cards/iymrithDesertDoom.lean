/-- Iymrith, Desert Doom -/
card iymrithDesertDoom : Spelled :=
  { name := "Iymrith, Desert Doom", cost := [generic 3, pip .blue, pip .blue],
    supertypes := [.legendary], types := [.creature], subtypes := [dragon],
    text :=
      [ flying,
        static (onlyWhile (abilityGrant thisCreature (ward [generic
            4]))
          (is it isUntapped)),
        whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
          (sequentially
            [ draw 1,
              doIf (compareAmt (countOf (cardIn yourHand)) .less 3)
                (draw theDifference) ]) ],
    power := stat 5, toughness := stat 5 }
