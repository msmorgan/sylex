keyword provoke : Ability := {
  lexical := { lemma := "provoke" },
  spelling := "provoke",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.attack Option.none))
        (Semantics.Templates.Action.may
          (Semantics.Templates.Action.establishThroughout
            (Semantics.Templates.StaticSpec.must
              (Semantics.Templates.Subject.actor
                (Semantics.Templates.NounPhrase.target
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.isCreature,
                      Semantics.Templates.Predicate.controlledBy Semantics.Templates.NounPhrase.defendingPlayer])))
              (Semantics.Templates.Action.block Semantics.Templates.NounPhrase.thisCreature))
            Semantics.Templates.Duration.untilEndOfCombat)
          (Option.some (Semantics.Definitions.untap (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.creature))))]
}
