/-- "Melee": "Whenever this creature attacks, it gets +1/+1 until end of turn for each opponent
you attacked with a creature this combat." [CR#702.121a]: a count of opponents, each attacked by
any number of creatures, not of attack events. -/
keyword melee : Ability := {
  lexical := { lemma := "melee" },
  spelling := "melee",
  body :=
    [ whenever (does thisCreature (.attack none))
        (get thisCreature
          (.up (forEach 1 (and [isOpponent,
            did you (.attackWith (a isCreature) (some .which)) thisCombat])))
          (.up (forEach 1 (and [isOpponent,
            did you (.attackWith (a isCreature) (some .which)) thisCombat])))
          untilEndOfTurn) ]
}
