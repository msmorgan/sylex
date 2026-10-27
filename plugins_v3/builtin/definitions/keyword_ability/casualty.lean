keyword casualty (amount : Amount) : Ability := {
  lexical := { lemma := "casualty", parameters := [[Amount]] },
  spelling := "casualty <amount>",
  body :=
    [ static
        (.addedCost
          (.perform (sacrifice (a (and [isCreature, hasPower .atLeast amount]))))
          true .once),
      when (does you (cast thisSpell))
        (if_ := keywordCostPaid casualty thisSpell)
        (sequentially
          [ copyOnStack thisSpell,
            may (.chooseNewTargets (that .copy)) ]) ]
}
