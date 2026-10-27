action recruit : Action := {
  lexical := { verbs := [{ lemma := "recruit", frames := [Intransitive] }] },
  spelling := "recruit",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.draw 1,
        Semantics.Definitions.discard
          (Semantics.Templates.NounPhrase.a (Semantics.Templates.Predicate.cardIn (Semantics.Templates.Location.handOf Semantics.Templates.NounPhrase.actor))),
        Semantics.Templates.Action.doIf
          (Semantics.Templates.Condition.not
            (Semantics.Templates.Condition.is
              (Semantics.Templates.NounPhrase.the
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.isCard,
                    Semantics.Templates.Predicate.was (Semantics.Definitions.discard Semantics.Templates.NounPhrase.which)
                      (Option.some Semantics.Templates.LookbackSpan.thisWay)]))
              Semantics.Templates.Predicate.isLand))
          (Semantics.Definitions.create 1
            (Semantics.Templates.TokenSpec.written
              (Semantics.Templates.CharacteristicBundle.creatureToken 1 1 [Semantics.Color.white]
                [Semantics.Definitions.human, Semantics.Definitions.soldier])))]
}
