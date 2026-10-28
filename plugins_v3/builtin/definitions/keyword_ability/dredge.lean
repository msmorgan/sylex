keyword dredge (amount : Amount) : Ability := {
  lexical := { lemma := "dredge", parameters := [[Amount]] },
  spelling := "dredge <amount>",
  body :=
    [static
        (onlyWhile
          (replace
            (would (actor you) (draw 1))
            (instead
              (may
                (sequentially
                  [mill amount,
                    returnToHand this_ yourGraveyard]))))
          (compareAmt
            (countOf (cardIn yourLibrary))
            .atLeast amount))]
}
