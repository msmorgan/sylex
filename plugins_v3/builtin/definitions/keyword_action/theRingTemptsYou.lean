/-- "The Ring tempts you": the Ring emblem if you have none, then you choose a creature you
control as your Ring-bearer (`Definition`, [CR#701.54a,701.54c]). The emblem's abilities count
the times the Ring has tempted you by the deed's own steps, `tempting`. -/
action theRingTemptsYou : Action := {
  lexical := { verbs := [{ lemma := "tempt", frames := [Monotransitive] }], nouns := ["Ring"] },
  spelling := "The Ring tempts you",
  body :=
    have ringBearers :=
      and
        [hasDesignation ringBearer, youControl];
    have tempting :=
      [choose
          (a
            (and [isCreature, actorControls])),
        gainDesignation (that creature)
          ringBearer];
    have timesTempted :=
      eventCount
        (.ownScope (some (.action "theRingTemptsYou"))
          (sequentially tempting))
        (actor you) thisGame;
    sequentially
      (doIf
          (not
            (exists_
              (and
                [isEmblem, named (printedName "The Ring"),
                  ownedBy theActor])))
          (getNamedEmblem "The Ring"
            [static
                (characteristicChange (allOf ringBearers)
                  [typeLine adds
                      { supertypes := some [legendary] }]),
              static
                (cant
                  (by_
                    (allOf
                      (and
                        [isCreature,
                          hasPower greater
                            (powerOf (a ringBearers))])))
                  (block (allOf ringBearers))),
              static
                (onlyWhile
                  (abilityGrant this_
                    (whenever
                      (does (actor (a ringBearers))
                        attack)
                      (sequentially
                        [draw 1,
                          discard
                            (a (cardIn hand))])))
                  (compareAmt timesTempted atLeast 2)),
              static
                (onlyWhile
                  (abilityGrant this_
                    (whenever
                      (does
                        (by_ (a isCreature))
                        (block (a ringBearers)))
                      (atNext endOfCombat
                        (controllerSacrifices
                          (that creature)))))
                  (compareAmt timesTempted atLeast 3)),
              static
                (onlyWhile
                  (abilityGrant this_
                    (whenever
                      (does (actor (a ringBearers))
                        (dealDamage anyAmount
                          (a isPlayer) (some combat)))
                      (instruct eachOpponent (loseLife 3))))
                  (compareAmt timesTempted atLeast 4))]) ::
        tempting)
}
