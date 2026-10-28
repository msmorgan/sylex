/-- "Flanking" means "Whenever this creature becomes blocked by a creature without flanking, the
blocking creature gets −1/−1 until end of turn." [CR#702.25a] -/
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
