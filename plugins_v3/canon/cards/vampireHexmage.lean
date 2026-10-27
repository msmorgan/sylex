/-- Vampire Hexmage -/
card vampireHexmage : Spelled :=
  { name := "Vampire Hexmage", cost := [pip .black, pip .black], types := [.creature],
    subtypes := [vampire, shaman],
    text :=
      [ firstStrike,
        activated (sacrifice thisCreature) (removeAllCounters none
            (target isPermanent)) ],
    power := stat 2, toughness := stat 1 }
