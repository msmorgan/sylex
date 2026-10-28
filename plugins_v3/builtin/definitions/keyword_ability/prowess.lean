keyword prowess : Ability := {
  lexical := { lemma := "prowess" },
  spelling := "prowess",
  body :=
    [whenever
        (does (actor you)
          (cast
            (a
              (and
                [in_ stack,
                  non isCreature]))))
        (get thisCreature (up 1) (up 1)
          (some untilEndOfTurn))]
}
