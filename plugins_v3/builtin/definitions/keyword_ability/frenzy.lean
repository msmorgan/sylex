/-- "Frenzy N": "Whenever this creature attacks and isn't blocked, it gets +N/+0 until end of
turn." [CR#702.68a] -/
keyword frenzy (amount : Amount) : Ability := {
  lexical := { lemma := "frenzy", parameters := [[Amount]] },
  spelling := "frenzy <amount>",
  body :=
    [ whenever (does thisCreature (.attackUnblocked none))
        (get thisCreature (.up amount) (.up 0) untilEndOfTurn) ]
}
