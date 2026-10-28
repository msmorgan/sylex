/-- "Afflict N" means "Whenever this creature becomes blocked, defending player loses N life."
[CR#702.130a] -/
keyword afflict (amount : Amount) : Ability := {
  lexical := { lemma := "afflict", parameters := [[Amount]] },
  spelling := "afflict <amount>",
  body :=
    [ whenever (does any (becomeBlocked thisCreature))
        (instruct defendingPlayer (loseLife amount)) ]
}
