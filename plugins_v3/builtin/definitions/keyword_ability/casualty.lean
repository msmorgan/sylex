keyword casualty (amount : Amount) : Ability := {
  lexical := { lemma := "casualty", parameters := [[Amount]] },
  spelling := "casualty <amount>",
  body :=
    [static
        (addedCost
          (perform
            (sacrifice
              (a
                (and
                  [isCreature, hasPower .atLeast amount]))))
          true .once),
      when
        (does (actor you)
          (cast thisSpell))
        (sequentially
          [copyOnStack thisSpell,
            may (chooseNewTargets (that .copy))])
        [] [] none none none
        (some
          (keywordCostPaid casualty.label thisSpell))]
}
