card felidarSovereign : Spelled :=
  { name := "Felidar Sovereign", cost := [generic 4, pip .white, pip .white],
    types := [.creature], subtypes := [cat, beast],
    text :=
      [ vigilance, lifelink,
        at_ (beginningOfYour .upkeep)
          (if_ := compareAmt (lifeTotalOf you) .atLeast 40)
          winGame ],
    power := stat 4, toughness := stat 6 }
