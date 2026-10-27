keyword horsemanship : Ability := {
  lexical := { lemma := "horsemanship" },
  spelling := "horsemanship",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant
          (Semantics.Templates.Subject.by_
            (Semantics.Templates.NounPhrase.allOf
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isCreature,
                  Semantics.Templates.Predicate.not
                    (Semantics.Templates.Predicate.hasKeyword
                      (Semantics.Templates.KeywordTerm.theKeyword Semantics.Definitions.horsemanship.label))])))
          (Semantics.Templates.Action.block Semantics.Templates.NounPhrase.thisCreature))]
}
