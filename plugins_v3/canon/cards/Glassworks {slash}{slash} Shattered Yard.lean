card := sharedLineSplit
  {
    shared := { types := [enchantment], subtypes := [room] },
    left := {
      name := "Glassworks",
      cost := [2, red],
      text := [
        when (does you (unlock thisDoor))
          (instruct thisRoom (dealDamage 4 (target [isCreature, anOpponentControls])))
      ]
    },
    right := {
      name := "Shattered Yard",
      cost := [4, red],
      text := [at_ (beginningOfYour endStep) (instruct thisRoom (dealDamage 1 eachOpponent))]
    }
  }
