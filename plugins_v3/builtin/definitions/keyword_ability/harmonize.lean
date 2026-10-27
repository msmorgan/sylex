keyword harmonize (cost : Cost) : Ability := {
  lexical := { lemma := "harmonize", parameters := [[Cost]] },
  spelling := "harmonize <cost>",
  body :=
    [ static
        (can you
          (cast this_ (from_ := yourGraveyard)
            (payment := .payingInstead
              (.compound
                [ cost,
                  .perform
                    (tap
                      (counted (upTo 1) (and [isUntapped, and [isCreature, youControl]]))) ])))),
      static
        (onlyWhile
          (.costShift thisSpell
            (.generic (.down (powerOf (the (and [was (tap which), isCreature])))) none))
          (keywordCostPaid harmonize thisSpell)),
      static
        (replace (would thisSpell (leave stack))
          (doIf (keywordCostPaid harmonize thisSpell) (instead (exile thisSpell)))) ]
}
