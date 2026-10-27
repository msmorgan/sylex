/-- "Cascade" with its reminder text [CR#702.85a]. -/
keyword cascade : Ability := {
  lexical := { lemma := "cascade", verbs := [{ lemma := "cascade", frames := [Intransitive] }] },
  spelling := "cascade",
  body :=
    let cheaper : Predicate :=
      .compare [.stat .manaValue] .less (.statOf (.stat .manaValue) thisSpell)
    [ when (does you (cast thisSpell))
          (sequentially
            [ exile (.run .top (and [non isLand, cheaper]) (libraryOf .you)),
              may (castWithoutPaying (that .card) cheaper),
              .move
                (allOf (and [isCard, was (exile which) thisWay, not (was (cast which) thisWay)]))
                .anyLocation onBottom .arrival [(.inOrder .randomOrder)] ]) ]
}
