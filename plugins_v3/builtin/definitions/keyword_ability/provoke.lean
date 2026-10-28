/-- "Provoke" means "Whenever this creature attacks, you may choose to have target creature
defending player controls block this creature this combat if able. If you do, untap that
creature." [CR#702.39a] -/
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
