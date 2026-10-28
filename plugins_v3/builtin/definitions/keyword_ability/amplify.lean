/-- "Amplify N" means "As this object enters, reveal any number of cards from your hand that share a
creature type with it. This permanent enters with N +1/+1 counters on it for each card revealed
this way. You can't reveal this card or any other cards that are entering the battlefield at the
same time as this card." [CR#702.38a] -/
keyword amplify (amount : Amount) : Ability := {
  lexical := { lemma := "amplify", parameters := [[Amount]] },
  spelling := "amplify <amount>",
  body :=
    [static
        (replace
          (would (actor thisPermanent)
            enter)
          (before
            (reveal
              (counted anyNumber
                (and
                  [cardIn yourHand,
                    shares (.subtype Semantics.CardType.creature)
                      share thisPermanent])))
            (some
              (alongside
                (putCounters
                  (forEach amount
                    (and
                      [isCard,
                        was (reveal which)
                          (some thisWay)]))
                  (printedKind p1p1Counter)
                  thisPermanent)))))]
}
