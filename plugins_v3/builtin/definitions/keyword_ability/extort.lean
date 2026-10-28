keyword extort : Ability := {
  lexical := { lemma := "extort" },
  spelling := "extort",
  body :=
    [whenever
        (does (actor you)
          (cast (a isSpell)))
        (may
          (pay
            (mana [hybridPip Semantics.Color.white Semantics.Color.black])
            Semantics.PayTimes.once)
          (some
            (sequentially
              [instruct eachOpponent (loseLife 1),
                gainLife thatMuch])))]
}
