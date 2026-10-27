card brazenDwarf : Spelled :=
  { name := "Brazen Dwarf", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [dwarf, shaman],
    text :=
      [ whenever (does you (rollDice anyAmount .anyDie))
          (instruct thisCreature (dealDamage 1 eachOpponent)) ],
    power := stat 1, toughness := stat 3 }
