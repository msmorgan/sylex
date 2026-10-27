keyword afflict (amount : Amount) : Ability := {
  lexical := { lemma := "afflict", parameters := [[Amount]] },
  spelling := "afflict <amount>",
  body :=
    [ whenever (does any (becomeBlocked thisCreature))
        (instruct defendingPlayer (loseLife amount)) ]
}
