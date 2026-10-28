keyword demonstrate : Ability := {
  lexical := { lemma := "demonstrate" },
  spelling := "demonstrate",
  body :=
    [when
        (does (actor you)
          (cast thisSpell))
        (may
          (sequentially
            [copyOnStack thisSpell,
              may
                (chooseNewTargets (that Semantics.NounWord.copy))])
          (some
            (sequentially
              [choose anOpponent,
                instruct (the isChosenPlayer)
                  (copyOnStack thisSpell),
                instruct (the isChosenPlayer)
                  (may
                    (chooseNewTargets (that Semantics.NounWord.copy)))])))]
}
