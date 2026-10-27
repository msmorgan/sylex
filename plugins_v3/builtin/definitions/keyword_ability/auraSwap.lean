keyword auraSwap (cost : Cost) : Ability := {
  lexical := { lemma := "Aura swap", parameters := [[Cost]] },
  spelling := "aura swap <cost>",
  body :=
    [ activated cost
        (may
          (exchangeCards thisPermanent
            (a (and [.isCard, .hasSubtype aura, .in_ yourHand])))) ]
}
