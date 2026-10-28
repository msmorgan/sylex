keyword harmonize (cost : Cost) : Ability := {
  lexical := { lemma := "harmonize", parameters := [[Cost]] },
  spelling := "harmonize <cost>",
  body :=
    [static
        (can (actor you)
          (cast this_ (some yourGraveyard)
            (payingInstead
              (compound
                [cost,
                  perform
                    (tap
                      (counted (upTo 1)
                        (and
                          [isUntapped,
                            and
                              [isCreature, youControl]])))])))),
      static
        (onlyWhile
          (costShift thisSpell
            (generic
              (down
                (powerOf
                  (the
                    (and
                      [was (tap which),
                        isCreature]))))
              none))
          (keywordCostPaid harmonize.label thisSpell)),
      static
        (replace
          (would (actor thisSpell)
            (leave stack))
          (doIf
            (keywordCostPaid harmonize.label thisSpell)
            (instead (exile thisSpell))))]
}
