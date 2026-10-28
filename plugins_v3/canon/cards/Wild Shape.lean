/-- *Wild Shape*: the header "Until end of turn, target creature you control has that base power
and toughness, becomes that creature type, and gains that ability." is the supplied body, written
once; each bullet is the creature shape it sets, as one set of the shape's characteristics: its
creature type, its base power and toughness [CR#613.4b] and its ability. -/
card := {
  name := "Wild Shape",
  cost := [green],
  types := [instant],
  text := [
    spell
      (chooseModesOver (exactly 1)
         ([
              { subtypes := [turtle], text := [hexproof], power := 1, toughness := 3 },
              { subtypes := [spider], text := [reach], power := 1, toughness := 5 },
              { subtypes := [elephant], text := [trample], power := 3, toughness := 3 }
            ]
            : List CharacteristicBundle)
         (fun shape => establish
            (qualityChange (target [isCreature, youControl]) sets (bundle shape)) untilEndOfTurn))
  ]
}
