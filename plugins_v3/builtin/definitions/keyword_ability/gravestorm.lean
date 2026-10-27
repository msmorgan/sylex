keyword gravestorm : Ability := {
  lexical := { lemma := "gravestorm" },
  spelling := "gravestorm",
  body :=
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisSpell))
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.copyOnStack Semantics.Templates.NounPhrase.thisSpell
              (Semantics.Templates.Amount.eventCount Semantics.Templates.Action.die
                (Semantics.Templates.Subject.actor (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isPermanent))
                Semantics.Templates.Duration.soFarThisTurn),
            Semantics.Templates.Action.may
              (Semantics.Templates.Action.chooseNewTargets (Semantics.Templates.NounPhrase.those Semantics.NounWord.copy))])]
}
