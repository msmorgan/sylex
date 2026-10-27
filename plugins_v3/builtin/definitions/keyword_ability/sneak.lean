keyword sneak (cost : Cost) : Ability := {
  lexical := { lemma := "sneak", parameters := [[Cost]] },
  spelling := "sneak <cost>",
  body :=
    let returnedCreaturesDefender :=
      the (and [ or [isPlayer, isPlaneswalker, isBattle],
                 inCombat .attackedBy (the (and [was (returnToHand which), isCreature])) ])
    [ static
          (can you
            (cast thisSpell
              (payment := payingInstead
                (compound
                  [ cost,
                    perform (returnToHand (a (and [isCreature, isUnblocked, youControl]))) ])))
            (timing := some (and [asInstant, duringPart .declareBlockers (some you)]))),
        -- It enters tapped and attacking what the returned creature was attacking [CR#702.190b].
        static
          (replace (would thisPermanent enter)
            (doIf (keywordCostPaid sneak thisPermanent)
              (withRiders
                [.entersAs .tapped, .entersAttacking (some returnedCreaturesDefender)]))) ]
}
