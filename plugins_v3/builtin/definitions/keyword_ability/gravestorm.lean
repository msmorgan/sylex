keyword gravestorm : Ability := {
  lexical := { lemma := "gravestorm" },
  spelling := "gravestorm",
  body :=
    [when
        (does (actor you)
          (cast thisSpell))
        (sequentially
          [copyOnStack thisSpell
              (eventCount die
                (actor (a isPermanent))
                soFarThisTurn),
            may
              (chooseNewTargets (those .copy))])]
}
