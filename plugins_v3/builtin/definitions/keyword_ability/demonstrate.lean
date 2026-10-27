keyword demonstrate : Ability := {
  lexical := { lemma := "demonstrate" },
  spelling := "demonstrate",
  body :=
    [ when (does you (cast thisSpell))
        (may (sequentially [copyOnStack thisSpell, may (.chooseNewTargets (that .copy))])
          (some
            (sequentially
              [ choose anOpponent,
                instruct (the isChosenPlayer) (copyOnStack thisSpell),
                instruct (the isChosenPlayer) (may (.chooseNewTargets (that .copy))) ]))) ]
}
