/-- "Prowess" means "Whenever you cast a noncreature spell, this creature gets +1/+1 until end of
turn." [CR#702.108a] -/
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
