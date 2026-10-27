keyword afterlife (amount : Amount) : Ability := {
  lexical := { lemma := "afterlife", parameters := [[Amount]] },
  spelling := "afterlife <amount>",
  body :=
    [ when (does thisPermanent die)
        (create amount (creatureToken 1 1 [.white, .black] [spirit] [flying])) ]
}
