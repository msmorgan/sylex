keyword haunt : Ability := {
  lexical := { lemma := "haunt", verbs := [{ lemma := "haunt", frames := [Monotransitive] }] },
  spelling := "haunt",
  body :=
    [Semantics.Templates.Ability.byCardClass
        (Semantics.Templates.Ability.when
          (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent) Semantics.Templates.Action.die)
          (Semantics.Templates.Action.sequentially
            [Semantics.Definitions.exile Semantics.Templates.NounPhrase.it,
              Semantics.Templates.Action.relate Semantics.Templates.NounPhrase.it Semantics.Definitions.haunting
                (Semantics.Templates.NounPhrase.target Semantics.Templates.Predicate.isCreature)]))
        (Semantics.Templates.Ability.when
          (Semantics.Templates.GameEvent.causes
            (Semantics.Templates.Causing.byEvent
              (Semantics.Templates.GameEvent.does Semantics.Templates.Subject.any
                (Semantics.Definitions.resolve Semantics.Templates.NounPhrase.thisSpell)))
            (Semantics.Templates.GameEvent.does Semantics.Templates.Subject.any
              (Semantics.Templates.Action.putInto Semantics.Templates.NounPhrase.thisSpell Semantics.Templates.Location.graveyard
                (Option.some Semantics.Templates.Location.stack))))
          (Semantics.Templates.Action.sequentially
            [Semantics.Definitions.exile Semantics.Templates.NounPhrase.it,
              Semantics.Templates.Action.relate Semantics.Templates.NounPhrase.it Semantics.Definitions.haunting
                (Semantics.Templates.NounPhrase.target Semantics.Templates.Predicate.isCreature)]))]
}
