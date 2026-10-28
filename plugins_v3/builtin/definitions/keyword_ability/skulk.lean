keyword skulk : Ability := {
  lexical := { lemma := "skulk" },
  spelling := "skulk",
  body :=
    [static
        (cant
          (by_
            (allOf
              (and
                [isCreature,
                  hasPower Semantics.Comparator.greater
                    (powerOf thisCreature)])))
          (block thisCreature))]
}
