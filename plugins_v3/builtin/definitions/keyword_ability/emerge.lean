keyword emerge (cost : Cost) : Ability := {
  lexical := { lemma := "emerge", parameters := [[Cost], [Quality (Some ("from")), Cost]] },
  spelling := "emerge <cost>",
  body :=
    [ static
        (altCost this_
          (some (.compound [cost, .perform (sacrifice (a isCreature))]))),
      static
        (onlyWhile
          (.costShift thisSpell
            (less
              (.statOf (.stat .manaValue) (the (and [was (sacrifice which), isCreature])))
              none))
          (keywordCostPaid emerge thisSpell)) ]
}
