keyword offspring (cost : Cost) : Ability := {
  lexical := { lemma := "offspring", parameters := [[Cost]] },
  spelling := "offspring <cost>",
  body :=
    [static (addedCost cost true .once),
      when
        (does (actor thisPermanent) enter)
        (create 1
          (copyOf thisPermanent
            [copyExceptEdits
                [characteristicEditStat .sets .power 1,
                  characteristicEditStat .sets .toughness 1]]))
        [] [] none none none
        (some
          (keywordCostPaid offspring.label thisPermanent))]
}
