/-- Autarch Mammoth -/
card autarchMammoth : Spelled :=
  { name := "Autarch Mammoth", cost := [generic 4, pip .green, pip .green], types := [.creature],
    subtypes := [elephant, mount],
    text := [(when (does thisCreature enter)
          (joins := [ joinedHeadWhile (does thisCreature attack)
              (whileTrue (is thisCreature (hasDesignation saddled))) ])
          (create 1 (creatureToken 3 3 [.green] [elephant]))), saddle 5],
    power := stat 5, toughness := stat 5 }
