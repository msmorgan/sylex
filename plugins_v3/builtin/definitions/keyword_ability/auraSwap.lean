/-- "Aura swap [cost]" means "[Cost]: You may exchange this permanent with an Aura card in your
hand." [CR#702.65a] -/
keyword auraSwap (cost : Cost) : Ability := {
  lexical := { lemma := "Aura swap", parameters := [[Cost]] },
  spelling := "aura swap <cost>",
  body :=
    [activated cost
        (may
          (exchangeCards thisPermanent
            (a
              (and
                [isCard, hasSubtype aura,
                  in_ yourHand]))))]
}
