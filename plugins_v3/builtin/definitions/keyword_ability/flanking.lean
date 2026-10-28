keyword flanking : Ability := {
  lexical := { lemma := "flanking" },
  spelling := "flanking",
  body :=
    [whenever
        (does
          (by_
            (a
              (and
                [isCreature,
                  not
                    (hasKeyword
                      (theKeyword flanking.label))])))
          (block thisCreature))
        (get (that creature) (down 1)
          (down 1) (some untilEndOfTurn))]
}
