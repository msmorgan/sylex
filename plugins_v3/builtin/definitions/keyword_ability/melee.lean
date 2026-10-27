/-- "Melee": "Whenever this creature attacks, it gets +1/+1 until end of turn for each opponent
you attacked with a creature this combat." [CR#702.121a]: a count of opponents, each attacked by
any number of creatures, not of attack events. -/
keyword melee : Ability := {
  lexical := { lemma := "melee" },
  spelling := "melee",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.attack Option.none))
        (Semantics.Templates.Action.get Semantics.Templates.NounPhrase.thisCreature
          (Semantics.Templates.up
            (Semantics.Templates.Amount.forEach 1
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isOpponent,
                  Semantics.Templates.Predicate.did (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
                    (Semantics.Templates.Action.attackWith (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isCreature)
                      (Option.some Semantics.Templates.NounPhrase.which))
                    (Option.some (Semantics.Templates.LookbackSpan.within Semantics.Templates.Duration.thisCombat))])))
          (Semantics.Templates.up
            (Semantics.Templates.Amount.forEach 1
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isOpponent,
                  Semantics.Templates.Predicate.did (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
                    (Semantics.Templates.Action.attackWith (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isCreature)
                      (Option.some Semantics.Templates.NounPhrase.which))
                    (Option.some (Semantics.Templates.LookbackSpan.within Semantics.Templates.Duration.thisCombat))])))
          (Option.some Semantics.Templates.Duration.untilEndOfTurn))]
}
