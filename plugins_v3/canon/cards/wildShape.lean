/-- *Wild Shape*: the header "Until end of turn, target creature you control has that base power
and toughness, becomes that creature type, and gains that ability." is the supplied body, written
once; each bullet is the creature shape it sets, as one set of the shape's characteristics: its
creature type, its base power and toughness [CR#613.4b] and its ability. -/
card wildShape : Spelled :=
  { name := "Wild Shape", cost := [pip .green], types := [.instant],
    text :=
      [ spell
          (chooseModesOver (exactly 1)
            ([ { characteristics :=
                  { subtypes := [turtle], text := [hexproof], power := stat 1,
                    toughness := stat 3 } },
              { characteristics :=
                  { subtypes := [spider], text := [reach], power := stat 1,
                    toughness := stat 5 } },
              { characteristics :=
                  { subtypes := [elephant], text := [trample], power := stat 3,
                    toughness := stat 3 } } ] : List CharacteristicBundle)
            (fun shape =>
              establish
                (qualityChange (target (and [isCreature, youControl])) .sets (bundle shape))
                untilEndOfTurn)) ] }
