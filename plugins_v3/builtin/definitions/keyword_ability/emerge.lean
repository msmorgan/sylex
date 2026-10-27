keyword emerge (cost : Cost) : Ability := {
  lexical := { lemma := "emerge", parameters := [[Cost], [Quality (Some ("from")), Cost]] },
  spelling := "emerge <cost>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.altCost Semantics.Templates.NounPhrase.this_
          (Option.some
            (Semantics.Templates.Cost.compound
              [cost,
                Semantics.Templates.Cost.perform
                  (Semantics.Definitions.sacrifice (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isCreature))]))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.costShift Semantics.Templates.NounPhrase.thisSpell
            (Semantics.Templates.CostShift.less
              (Semantics.Templates.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue)
                (Semantics.Templates.NounPhrase.the
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.was (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.which),
                      Semantics.Templates.Predicate.isCreature])))))
          (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.emerge.label Semantics.Templates.NounPhrase.thisSpell))]
}
