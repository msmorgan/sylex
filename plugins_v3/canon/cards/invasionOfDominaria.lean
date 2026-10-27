card invasionOfDominaria : Spelled := { body := .transforming
  { characteristics :=
    { name := "Invasion of Dominaria", cost := [generic 2, pip .white],
      types := [.battle], subtypes := [siege],
      text :=
        [ when (does thisSiege enter)
            (sequentially [gainLife 4, draw 1]) ],
      defense := stat 5 } }
  { characteristics :=
    { name := "Serra Faithkeeper", types := [.creature], subtypes := [angel],
      text := [flying, vigilance], power := stat 4, toughness := stat 4 } } }
