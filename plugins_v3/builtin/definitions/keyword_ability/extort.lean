keyword extort : Ability := {
  lexical := { lemma := "extort" },
  spelling := "extort",
  body :=
    [ whenever (does you (cast (a isSpell)))
        (may (pay (.mana [hybridPip .white .black]) .once)
          (sequentially [instruct eachOpponent (loseLife 1), gainLife .thatMuch])) ]
}
