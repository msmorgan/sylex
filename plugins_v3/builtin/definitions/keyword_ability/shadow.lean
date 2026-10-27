/-- "Shadow": "A creature with shadow can't be blocked by creatures without shadow, and a creature
without shadow can't be blocked by creatures with shadow." [CR#702.28b] Each half is about this
creature: creatures without shadow can't block it, and it can't block them. -/
keyword shadow : Ability := {
  lexical := { lemma := "shadow" },
  spelling := "shadow",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant
          (Semantics.Templates.Subject.by_
            (Semantics.Templates.NounPhrase.allOf
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isCreature,
                  Semantics.Templates.Predicate.not
                    (Semantics.Templates.Predicate.hasKeyword (Semantics.Templates.KeywordTerm.theKeyword Semantics.Definitions.shadow.label))])))
          (Semantics.Templates.Action.block Semantics.Templates.NounPhrase.thisCreature)),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant (Semantics.Templates.Subject.by_ Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.block
            (Semantics.Templates.NounPhrase.allOf
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isCreature,
                  Semantics.Templates.Predicate.not
                    (Semantics.Templates.Predicate.hasKeyword
                      (Semantics.Templates.KeywordTerm.theKeyword Semantics.Definitions.shadow.label))]))))]
}
