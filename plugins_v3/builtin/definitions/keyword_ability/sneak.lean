/-- "Sneak [cost]" means "Any time you could cast an instant during your declare blockers step, you
may cast this spell by paying [cost] and returning an unblocked creature you control to its
owner's hand rather than paying this spell's mana cost." [CR#702.190a] A permanent spell whose
sneak cost was paid enters tapped and attacking what the returned creature was attacking.
[CR#702.190b] -/
keyword sneak (cost : Cost) : Ability := {
  lexical := { lemma := "sneak", parameters := [[Cost]] },
  spelling := "sneak <cost>",
  body :=
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
                duringPart declareBlockers
                  (some you)]))),
      static
        (replace
          (would (actor thisPermanent)
            enter)
          (doIf
            (keywordCostPaid sneak.label thisPermanent)
            -- It enters tapped and attacking what the returned creature was attacking [CR#702.190b].
            (withRiders
              [entersAs tapped,
                entersAttacking
                  (some
                    (the
                      (and
                        [or
                            [isPlayer, isPlaneswalker, isBattle],
                          inCombat attackedBy
                            (some
                              (the
                                (and
                                  [was (returnToHand which),
                                    isCreature])))])))])))]
}
