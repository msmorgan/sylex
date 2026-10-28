/-- "Cascade" with its reminder text [CR#702.85a]. -/
keyword cascade : Ability := {
  lexical := { lemma := "cascade", verbs := [{ lemma := "cascade", frames := [Intransitive] }] },
  spelling := "cascade",
  body :=
    have cheaper :=
      compare less (statOf (.stat manaValue) thisSpell) [.stat manaValue];
    [when
        (does (actor you)
          (cast thisSpell))
        (sequentially
          [exile
              (run top
                (and [non isLand, cheaper])
                (libraryOf you)),
            may
              (castWithoutPaying (that card)
                (some cheaper)),
            move
              (allOf
                (and
                  [isCard,
                    was (exile which)
                      (some thisWay),
                    not
                      (was (cast which)
                        (some thisWay))]))
              wherever (some onBottom) .arrival
              [inOrder randomOrder]])]
}
