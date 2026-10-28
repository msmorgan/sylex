/-- "Cascade" with its reminder text [CR#702.85a]. -/
keyword cascade : Ability := {
  lexical := { lemma := "cascade", verbs := [{ lemma := "cascade", frames := [Intransitive] }] },
  spelling := "cascade",
  body :=
    have cheaper :=
      predicateCompare [Semantics.ProjAxis.stat Semantics.Stat.manaValue] Semantics.Comparator.less
        (statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue) thisSpell);
    [when
        (does (actor you)
          (cast thisSpell))
        (sequentially
          [exile
              (run Semantics.PileEnd.top
                (and [non isLand, cheaper])
                (libraryOf you)),
            may
              (castWithoutPaying (that Semantics.NounWord.card)
                (some cheaper)),
            actionMove
              (allOf
                (and
                  [isCard,
                    was (exile which)
                      (some thisWay),
                    not
                      (was (cast which)
                        (some thisWay))]))
              wherever (some onBottom) Semantics.ObservationPoint.arrival
              [inOrder Semantics.Arrangement.randomOrder]])]
}
