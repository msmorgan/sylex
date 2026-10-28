keyword conspire : Ability := {
  lexical := { lemma := "conspire" },
  spelling := "conspire",
  body :=
    [static
        (addedCost
          (perform
            (tap
              (counted (exactly 2)
                (and
                  [isUntapped, isCreature, youControl,
                    shares Semantics.QualitySort.color Semantics.Agreement.share
                      thisSpell]))))
          true Semantics.PayTimes.once),
      when
        (does (actor you)
          (cast thisSpell))
        (sequentially
          [copyOnStack thisSpell,
            may (chooseNewTargets (that Semantics.NounWord.copy))])
        [] [] none none none
        (some
          (keywordCostPaid conspire.label thisSpell))]
}
