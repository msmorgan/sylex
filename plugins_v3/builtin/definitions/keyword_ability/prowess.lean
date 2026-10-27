keyword prowess : Ability := {
  lexical := { lemma := "prowess" },
  spelling := "prowess",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast
            (Semantics.Templates.NounPhrase.a
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.in_ Semantics.Templates.Location.stack,
                  Semantics.Templates.Predicate.non Semantics.Templates.Predicate.isCreature]))))
        (Semantics.Templates.Action.get Semantics.Templates.NounPhrase.thisCreature (Semantics.Templates.up 1) (Semantics.Templates.up 1)
          (Option.some Semantics.Templates.Duration.untilEndOfTurn))]
}
