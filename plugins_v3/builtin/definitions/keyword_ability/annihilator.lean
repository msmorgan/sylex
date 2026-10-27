keyword annihilator (amount : Amount) : Ability := {
  lexical := { lemma := "annihilator", parameters := [[Amount]] },
  spelling := "annihilator <amount>",
  body :=
    [ whenever (does thisCreature (.attack none))
        (instruct defendingPlayer
          (sacrifice (counted (.exactlyOf amount) isPermanent))) ]
}
