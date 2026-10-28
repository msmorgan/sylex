keyword epic : Ability := {
  lexical := { lemma := "epic" },
  spelling := "epic",
  body :=
    [spell
        (establishThroughout
          (cant (actor you)
            (cast Templates.NounPhrase.any))
          restOfGame),
      spell
        (delayWithin (beginningOfYour .upkeep)
          restOfGame
          (sequentially
            [copy .fromStack thisSpell 1
                [copyExceptEdits
                    [characteristicEditRemovedAbilities
                        (abilitySelectionSpecified
                          [abilities (theKeyword epic.label)])]],
              may
                (chooseNewTargets (that .copy))]))]
}
