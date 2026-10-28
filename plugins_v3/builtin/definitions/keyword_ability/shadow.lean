/-- "Shadow": "A creature with shadow can't be blocked by creatures without shadow, and a creature
without shadow can't be blocked by creatures with shadow." [CR#702.28b] Each half is about this
creature: creatures without shadow can't block it, and it can't block them. -/
keyword shadow : Ability := {
  lexical := { lemma := "shadow" },
  spelling := "shadow",
  body :=
    [static
        (cant
          (by_
            (allOf
              (and
                [isCreature,
                  not
                    (hasKeyword (theKeyword shadow.label))])))
          (block thisCreature)),
      static
        (cant (by_ thisCreature)
          (block
            (allOf
              (and
                [isCreature,
                  not
                    (hasKeyword
                      (theKeyword shadow.label))]))))]
}
