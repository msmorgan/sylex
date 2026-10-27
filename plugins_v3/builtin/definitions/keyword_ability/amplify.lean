keyword amplify (amount : Amount) : Ability := {
  lexical := { lemma := "amplify", parameters := [[Amount]] },
  spelling := "amplify <amount>",
  body :=
    [ static
        (replace (would thisPermanent enter)
          (before
            (reveal
              (counted anyNumber
                (and [cardIn yourHand, .shares (.subtype .creature) .share thisPermanent])))
            (some
              (alongside
                (putCounters (forEach amount (and [.isCard, was (reveal which) thisWay]))
                  p1p1Counter thisPermanent))))) ]
}
