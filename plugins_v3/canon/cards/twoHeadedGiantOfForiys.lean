/-- Two-Headed Giant of Foriys -/
card twoHeadedGiantOfForiys : Spelled :=
  { name := "Two-Headed Giant of Foriys", cost := [generic 4, pip .red],
    types := [.creature], subtypes := [giant],
    text := [trample, static
               (can thisCreature (block (counted (additional (exactly 1)) isCreature))
                  (timing := eachCombat))],
    power := stat 4, toughness := stat 4 }
