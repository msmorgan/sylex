keyword provoke : Ability := {
  lexical := { lemma := "provoke" },
  spelling := "provoke",
  body :=
    [whenever
        (does (actor thisCreature)
          (attack none))
        (may
          (establishThroughout
            (must
              (actor
                (target
                  (and
                    [isCreature,
                      controlledBy defendingPlayer])))
              (block thisCreature))
            untilEndOfCombat)
          (some (untap (that creature))))]
}
