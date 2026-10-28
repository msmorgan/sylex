keyword transfigure (cost : Cost) : Ability := {
  lexical := { lemma := "transfigure", parameters := [[Cost]] },
  spelling := "transfigure <cost>",
  body :=
    [activatedOnlyDuring
        (compound
          [cost, perform (sacrifice thisPermanent)])
        (sequentially
          [search yourLibrary
              (a
                (and
                  [isCard, isCreature,
                    hasManaValue Semantics.Comparator.eq
                      (statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue)
                        thisPermanent)])),
            putOntoBattlefield (that Semantics.NounWord.card),
            shuffle])
        asSorcery]
}
