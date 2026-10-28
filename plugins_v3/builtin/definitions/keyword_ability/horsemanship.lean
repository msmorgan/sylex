keyword horsemanship : Ability := {
  lexical := { lemma := "horsemanship" },
  spelling := "horsemanship",
  body :=
    [static
        (cant
          (by_
            (allOf
              (and
                [isCreature,
                  not
                    (hasKeyword
                      (theKeyword horsemanship.label))])))
          (block thisCreature))]
}
