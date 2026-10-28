keyword ascend : Ability := {
  lexical := { lemma := "ascend" },
  spelling := "ascend",
  body :=
    [byCardClass
        (static
          (whileHolds
            (and
              [compareAmt
                  (countOf
                    (and [isPermanent, youControl]))
                  Semantics.Comparator.atLeast 10,
                not
                  (is you
                    (hasDesignation citysBlessing))])
            (gainDesignation you citysBlessing
              (some restOfGame))))
        (spell
          (doIf
            (and
              [compareAmt
                  (countOf
                    (and [isPermanent, youControl]))
                  Semantics.Comparator.atLeast 10,
                not
                  (is you
                    (hasDesignation citysBlessing))])
            (gainDesignation you citysBlessing
              (some restOfGame))))]
}
