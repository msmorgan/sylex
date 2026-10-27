keyword offspring (cost : Cost) : Ability := {
  lexical := { lemma := "offspring", parameters := [[Cost]] },
  spelling := "offspring <cost>",
  body :=
    [ static (.addedCost cost true .once),
      when (does thisPermanent enter)
        (if_ := keywordCostPaid offspring thisPermanent)
        (create 1
          (.copyOf thisPermanent
            [.edits [.stat .sets .power 1, .stat .sets .toughness 1]])) ]
}
