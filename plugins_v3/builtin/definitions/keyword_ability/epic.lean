keyword epic : Ability := {
  lexical := { lemma := "epic" },
  spelling := "epic",
  body :=
    [ spell
        (establishThroughout (cant you (cast .any)) restOfGame),
      spell
        (delayWithin (beginningOfYour .upkeep) restOfGame
          (sequentially
            [ .copy .fromStack thisSpell 1
                [.edits [.removedAbilities (.specified [.term epic])]],
              may (.chooseNewTargets (that .copy)) ])) ]
}
