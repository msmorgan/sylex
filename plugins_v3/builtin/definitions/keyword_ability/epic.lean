keyword epic : Ability := {
  lexical := { lemma := "epic" },
  spelling := "epic",
  body :=
    [spell
        (establishThroughout
          (cant (actor you)
            (cast NounPhrase.any))
          restOfGame),
      spell
        (delayWithin (beginningOfYour Semantics.TurnPart.upkeep)
          restOfGame
          (sequentially
            [copy Semantics.CopySort.fromStack thisSpell 1
                [copyExceptEdits
                    [characteristicEditRemovedAbilities
                        (abilitySelectionSpecified
                          [abilities (theKeyword epic.label)])]],
              may
                (chooseNewTargets (that Semantics.NounWord.copy))]))]
}
