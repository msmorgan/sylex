/-- "Bushido N" with its definition, a triggered ability [CR#702.45a]. -/
keyword bushido (count : Amount) : Ability := {
  lexical := { lemma := "bushido", parameters := [[Amount]] },
  spelling := "bushido <count>",
  body :=
    [ whenever (does thisCreature block) (or_ := [does any (becomeBlocked thisCreature)])
        (get thisCreature (.up count) (.up count) untilEndOfTurn) ]
}
