/-- "The Ring tempts you": the Ring emblem if you have none, then you choose a creature you
control as your Ring-bearer (`Definition`, [CR#701.54a,701.54c]). The emblem's abilities count
the times the Ring has tempted you by the deed's own steps, the choice and the designation. -/
action theRingTemptsYou : Action := {
  lexical := { verbs := [{ lemma := "tempt", frames := [Monotransitive] }], nouns := ["Ring"] },
  spelling := "The Ring tempts you",
  body :=
    sequentially
      [doIf
          (not
            (exists_
              (and
                [isEmblem, named (printedName "The Ring"),
                  ownedBy theActor])))
          (getNamedEmblem "The Ring"
            [static
                (characteristicChange (allOf (and [hasDesignation ringBearer, youControl]))
                  [typeLine adds
                      { supertypes := some [legendary] }]),
              static
                (cant
                  (by_
                    (allOf
                      (and
                        [isCreature,
                          hasPower greater
                            (powerOf (a (and [hasDesignation ringBearer, youControl])))])))
                  (block (allOf (and [hasDesignation ringBearer, youControl])))),
              static
                (onlyWhile
                  (abilityGrant this_
                    (whenever
                      (does (actor (a (and [hasDesignation ringBearer, youControl])))
                        attack)
                      (sequentially
                        [draw 1,
                          discard
                            (a (cardIn hand))])))
                  (compareAmt
                    (eventCount
                        (.ownScope (some (.action "theRingTemptsYou"))
                          (sequentially
                            [choose (a (and [isCreature, actorControls])),
                              gainDesignation (that creature) ringBearer]))
                        (actor you) thisGame)
                    atLeast 2)),
              static
                (onlyWhile
                  (abilityGrant this_
                    (whenever
                      (does
                        (by_ (a isCreature))
                        (block (a (and [hasDesignation ringBearer, youControl]))))
                      (atNext endOfCombat
                        (controllerSacrifices
                          (that creature)))))
                  (compareAmt
                    (eventCount
                        (.ownScope (some (.action "theRingTemptsYou"))
                          (sequentially
                            [choose (a (and [isCreature, actorControls])),
                              gainDesignation (that creature) ringBearer]))
                        (actor you) thisGame)
                    atLeast 3)),
              static
                (onlyWhile
                  (abilityGrant this_
                    (whenever
                      (does (actor (a (and [hasDesignation ringBearer, youControl])))
                        (dealDamage anyAmount
                          (a isPlayer) (some combat)))
                      (instruct eachOpponent (loseLife 3))))
                  (compareAmt
                    (eventCount
                        (.ownScope (some (.action "theRingTemptsYou"))
                          (sequentially
                            [choose (a (and [isCreature, actorControls])),
                              gainDesignation (that creature) ringBearer]))
                        (actor you) thisGame)
                    atLeast 4))]),
        choose (a (and [isCreature, actorControls])),
        gainDesignation (that creature) ringBearer]
}
