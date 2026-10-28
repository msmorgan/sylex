/-- The Ninja enters attacking what the returned creature was attacking [CR#702.49c]. -/
keyword ninjutsu (cost : Cost) : Ability := {
  lexical := {
    lemma := "ninjutsu",
    parameters := [[Cost]],
    variants := [{ lemma := "commander ninjutsu", parameters := [[Cost]] }]
  },
  spelling := "ninjutsu <cost>",
  body :=
    have returnedCreaturesDefender :=
      the
        (and
          [or
              [isPlayer, isPlaneswalker, isBattle],
            inCombat Semantics.CombatRelation.attackedBy
              (some
                (the
                  (and
                    [was (returnToHand which),
                      isCreature])))]);
    [activated
        (compound
          [cost, perform (reveal this_),
            perform
              (returnToHand
                (a
                  (and
                    [isCreature, isAttacking, isUnblocked,
                      youControl])))])
        (putOntoBattlefieldTappedAttacking this_ yourHand
          (some returnedCreaturesDefender))]
}
