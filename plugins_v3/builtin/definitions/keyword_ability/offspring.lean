keyword offspring (cost : Cost) : Ability := {
  lexical := { lemma := "offspring", parameters := [[Cost]] },
  spelling := "offspring <cost>",
  body :=
    [static (addedCost cost true Semantics.PayTimes.once),
      when
        (does (actor thisPermanent) enter)
        (create 1
          (copyOf thisPermanent
            [copyExceptEdits
                [characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.power 1,
                  characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness 1]]))
        [] [] none none none
        (some
          (keywordCostPaid offspring.label thisPermanent))]
}
