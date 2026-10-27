keyword provoke : Ability := {
  lexical := { lemma := "provoke" },
  spelling := "provoke",
  body :=
    [ whenever (does thisCreature (.attack none))
        (may
          (establishThroughout
            (must (target (and [isCreature, controlledBy defendingPlayer]))
              (.block thisCreature))
            untilEndOfCombat)
          (untap (that creature))) ]
}
