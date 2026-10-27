keyword gravestorm : Ability := {
  lexical := { lemma := "gravestorm" },
  spelling := "gravestorm",
  body :=
    [ when (does you (cast thisSpell))
        (sequentially
          [ copyOnStack thisSpell
              (eventCount die (a isPermanent) soFarThisTurn),
            may (.chooseNewTargets (those .copy)) ]) ]
}
