card orochiEggwatcher : Spelled := { body := .flip
  { characteristics :=
    { name := "Orochi Eggwatcher", cost := [generic 2, pip .green], types := [.creature],
      subtypes := [snake, shaman],
      text :=
        [ activated (compound [[generic 2, pip .green], tapSymbol])
            (sequentially
              [ create 1 (creatureToken 1 1 [.green] [snake]),
                doIf (compareAmt (countOf (and [isCreature, youControl])) .atLeast 10)
                  (flip thisCreature) ]) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Shidako, Broodmistress", supertypes := [.legendary], types := [.creature],
      subtypes := [snake, shaman],
      text :=
        [ activated (compound [[pip .green], sacrifice (a isCreature)])
            (get (target isCreature) (up 3) (up 3) untilEndOfTurn) ],
      power := stat 3, toughness := stat 3 } } }
