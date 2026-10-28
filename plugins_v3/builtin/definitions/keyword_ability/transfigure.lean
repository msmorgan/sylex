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
                    hasManaValue .eq
                      (statOf (.stat .manaValue)
                        thisPermanent)])),
            putOntoBattlefield (that .card),
            shuffle])
        asSorcery]
}
