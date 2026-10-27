keyword rampage (amount : Amount) : Ability := {
  lexical := { lemma := "rampage", parameters := [[Amount]] },
  spelling := "rampage <amount>",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does Semantics.Templates.Subject.any
          (Semantics.Templates.Action.becomeBlocked Semantics.Templates.NounPhrase.thisCreature))
        (Semantics.Templates.Action.get Semantics.Templates.NounPhrase.thisCreature
          (Semantics.Templates.up
            (Semantics.Templates.Amount.times amount
              (Semantics.Templates.Amount.minus
                (Semantics.Templates.Amount.countOf
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.isCreature,
                      Semantics.Templates.Predicate.inCombat Semantics.CombatRelation.blockerOf
                        (Option.some Semantics.Templates.NounPhrase.thisCreature)]))
                1)))
          (Semantics.Templates.up
            (Semantics.Templates.Amount.times amount
              (Semantics.Templates.Amount.minus
                (Semantics.Templates.Amount.countOf
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.isCreature,
                      Semantics.Templates.Predicate.inCombat Semantics.CombatRelation.blockerOf
                        (Option.some Semantics.Templates.NounPhrase.thisCreature)]))
                1)))
          (Option.some Semantics.Templates.Duration.untilEndOfTurn))]
}
