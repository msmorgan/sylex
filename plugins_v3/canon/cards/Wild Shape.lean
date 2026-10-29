/-- *Wild Shape*: each mode writes out the printed header, "Until end of turn, target creature you
control has that base power and toughness, becomes that creature type, and gains that ability.",
with its bullet's creature shape as one set of characteristics: its creature type, its base power
and toughness [CR#613.4b] and its ability. -/
card := {
  name := "Wild Shape",
  cost := [green],
  types := [instant],
  text := [
    spell
      (chooseModes (exactly 1)
         [
           establish
             (qualityChange (target [isCreature, youControl]) sets
                (bundle { subtypes := [turtle], text := [hexproof], power := 1, toughness := 3 }))
             untilEndOfTurn,
           establish
             (qualityChange (target [isCreature, youControl]) sets
                (bundle { subtypes := [spider], text := [reach], power := 1, toughness := 5 }))
             untilEndOfTurn,
           establish
             (qualityChange (target [isCreature, youControl]) sets
                (bundle { subtypes := [elephant], text := [trample], power := 3, toughness := 3 }))
             untilEndOfTurn
         ])
  ]
}
