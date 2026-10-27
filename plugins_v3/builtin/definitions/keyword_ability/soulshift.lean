keyword soulshift (amount : Amount) : Ability := {
  lexical := { lemma := "soulshift", parameters := [[Amount]] },
  spelling := "soulshift <amount>",
  body :=
    [ when (does thisPermanent die)
        (may
          (returnToHand
            (target
              (and
                [ .isCard,
                  .hasSubtype spirit,
                  hasManaValue .atMost amount,
                  .in_ yourGraveyard ]))
            yourGraveyard)) ]
}
