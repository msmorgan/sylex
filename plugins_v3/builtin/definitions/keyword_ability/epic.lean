keyword epic : Ability := {
  lexical := { lemma := "epic" },
  spelling := "epic",
  body :=
    [Semantics.Templates.Ability.spell
        (Semantics.Templates.Action.establishThroughout
          (Semantics.Templates.StaticSpec.cant (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
            (Semantics.Definitions.cast Semantics.Templates.NounPhrase.any))
          Semantics.Templates.Duration.restOfGame),
      Semantics.Templates.Ability.spell
        (Semantics.Templates.Action.delayWithin (Semantics.Templates.GameEvent.beginningOfYour Semantics.TurnPart.upkeep)
          Semantics.Templates.Duration.restOfGame
          (Semantics.Templates.Action.sequentially
            [Semantics.Templates.Action.copy Semantics.CopySort.fromStack Semantics.Templates.NounPhrase.thisSpell 1
                [Semantics.Templates.CopyExcept.copyExceptEdits
                    [Semantics.Templates.CharacteristicEdit.characteristicEditRemovedAbilities
                        (Semantics.Templates.AbilitySelection.abilitySelectionSpecified
                          [Semantics.Templates.AbilityLost.abilities (Semantics.Templates.KeywordTerm.theKeyword Semantics.Definitions.epic.label)])]],
              Semantics.Templates.Action.may
                (Semantics.Templates.Action.chooseNewTargets (Semantics.Templates.NounPhrase.that Semantics.NounWord.copy))]))]
}
