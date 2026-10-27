keyword demonstrate : Ability := {
  lexical := { lemma := "demonstrate" },
  spelling := "demonstrate",
  body :=
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisSpell))
        (Semantics.Templates.Action.may
          (Semantics.Templates.Action.sequentially
            [Semantics.Templates.Action.copyOnStack Semantics.Templates.NounPhrase.thisSpell,
              Semantics.Templates.Action.may
                (Semantics.Templates.Action.chooseNewTargets (Semantics.Templates.NounPhrase.that Semantics.NounWord.copy))])
          (Option.some
            (Semantics.Templates.Action.sequentially
              [Semantics.Templates.Action.choose Semantics.Templates.NounPhrase.anOpponent,
                Semantics.Templates.Action.instruct (Semantics.Templates.NounPhrase.the Semantics.Templates.Predicate.isChosenPlayer)
                  (Semantics.Templates.Action.copyOnStack Semantics.Templates.NounPhrase.thisSpell),
                Semantics.Templates.Action.instruct (Semantics.Templates.NounPhrase.the Semantics.Templates.Predicate.isChosenPlayer)
                  (Semantics.Templates.Action.may
                    (Semantics.Templates.Action.chooseNewTargets (Semantics.Templates.NounPhrase.that Semantics.NounWord.copy)))])))]
}
