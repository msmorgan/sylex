keyword extort : Ability := {
  lexical := { lemma := "extort" },
  spelling := "extort",
  body :=
    [whenever
        (does (actor you)
          (cast (a isSpell)))
        (may
          (pay
            (mana [hybridPip .white .black])
            .once)
          (some
            (sequentially
              [instruct eachOpponent (loseLife 1),
                gainLife thatMuch])))]
}
