keyword harmonize (cost : Cost) : Ability := {
  lexical := { lemma := "harmonize", parameters := [[Cost]] },
  spelling := "harmonize <cost>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.this_ (Option.some Semantics.Templates.Location.yourGraveyard)
            (Semantics.Templates.PlayPayment.payingInstead
              (Semantics.Templates.Cost.compound
                [cost,
                  Semantics.Templates.Cost.perform
                    (Semantics.Definitions.tap
                      (Semantics.Templates.NounPhrase.counted (Semantics.Templates.Quantity.upTo 1)
                        (Semantics.Templates.Predicate.and
                          [Semantics.Templates.Predicate.isUntapped,
                            Semantics.Templates.Predicate.and
                              [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.youControl]])))])))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.costShift Semantics.Templates.NounPhrase.thisSpell
            (Semantics.Templates.CostShift.costShiftGeneric
              (Semantics.Templates.down
                (Semantics.Templates.Amount.powerOf
                  (Semantics.Templates.NounPhrase.the
                    (Semantics.Templates.Predicate.and
                      [Semantics.Templates.Predicate.was (Semantics.Definitions.tap Semantics.Templates.NounPhrase.which),
                        Semantics.Templates.Predicate.isCreature]))))
              Option.none))
          (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.harmonize.label Semantics.Templates.NounPhrase.thisSpell)),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisSpell)
            (Semantics.Templates.Action.leave Semantics.Templates.Location.stack))
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.harmonize.label Semantics.Templates.NounPhrase.thisSpell)
            (Semantics.Templates.Replacement.instead (Semantics.Definitions.exile Semantics.Templates.NounPhrase.thisSpell))))]
}
