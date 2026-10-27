card undercoverButler : Spelled :=
  { name := "Undercover Butler", cost := [generic 2, hybridPip .blue .black],
    types := [.creature], subtypes := [human, rogue],
    text :=
      [ whenever
          (does thisCreature
             (attack
                   (a (and [isPlayer, superlative .max (.playerStat .lifeTotal) isPlayer]))))
          (establishThroughout (cant any (block it)) thisTurn) ],
    power := stat 2, toughness := stat 3 }
