keyword flanking : Ability := {
  lexical := { lemma := "flanking" },
  spelling := "flanking",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does
          (Semantics.Templates.Subject.by_
            (Semantics.Templates.NounPhrase.a
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isCreature,
                  Semantics.Templates.Predicate.not
                    (Semantics.Templates.Predicate.hasKeyword
                      (Semantics.Templates.KeywordTerm.theKeyword Semantics.Definitions.flanking.label))])))
          (Semantics.Templates.Action.block Semantics.Templates.NounPhrase.thisCreature))
        (Semantics.Templates.Action.get (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.creature) (Semantics.Templates.down 1)
          (Semantics.Templates.down 1) (Option.some Semantics.Templates.Duration.untilEndOfTurn))]
}
