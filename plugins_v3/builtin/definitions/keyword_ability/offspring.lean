keyword offspring (cost : Cost) : Ability := {
  lexical := { lemma := "offspring", parameters := [[Cost]] },
  spelling := "offspring <cost>",
  body :=
    [Semantics.Templates.Ability.static (Semantics.Templates.StaticSpec.addedCost cost Bool.true Semantics.PayTimes.once),
      Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent) Semantics.Templates.Action.enter)
        (Semantics.Definitions.create 1
          (Semantics.Templates.TokenSpec.copyOf Semantics.Templates.NounPhrase.thisPermanent
            [Semantics.Templates.CopyExcept.copyExceptEdits
                [Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.power 1,
                  Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness 1]]))
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.offspring.label Semantics.Templates.NounPhrase.thisPermanent))]
}
