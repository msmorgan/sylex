keyword sneak (cost : Cost) : Ability := {
  lexical := { lemma := "sneak", parameters := [[Cost]] },
  spelling := "sneak <cost>",
  body :=
    have returnedCreaturesDefender :=
      the
        (and
          [or
              [isPlayer, isPlaneswalker, isBattle],
            inCombat .attackedBy
              (some
                (the
                  (and
                    [was (returnToHand which),
                      isCreature])))]);
    [static
        (can (actor you)
          (cast thisSpell none
            (payingInstead
              (compound
                [cost,
                  perform
                    (returnToHand
                      (a
                        (and
                          [isCreature, isUnblocked,
                            youControl])))])))
          (some
            (and
              [asInstant,
                duringPart .declareBlockers
                  (some you)]))),
      static
        (replace
          (would (actor thisPermanent)
            enter)
          (doIf
            (keywordCostPaid sneak.label thisPermanent)
            -- It enters tapped and attacking what the returned creature was attacking [CR#702.190b].
            (withRiders
              [entersAs .tapped,
                entersAttacking (some returnedCreaturesDefender)])))]
}
