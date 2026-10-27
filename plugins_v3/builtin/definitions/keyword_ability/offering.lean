keyword offering (quality : Predicate) : Ability := {
  lexical := { lemma := "offering", parameters := [[QualityBefore]] },
  spelling := "<quality> offering",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.addedCost
          (Semantics.Templates.Cost.perform
            (Semantics.Definitions.sacrifice
              (Semantics.Templates.NounPhrase.a (Semantics.Templates.Predicate.and [quality, Semantics.Templates.Predicate.isPermanent]))))
          Bool.true Semantics.PayTimes.once),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.reducedBySymbols Semantics.Templates.NounPhrase.thisSpell
            (Semantics.Templates.ManaCostTerm.manaCostOf
              (Semantics.Templates.NounPhrase.the
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.was (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.which),
                    Semantics.Templates.Predicate.isPermanent]))))
          (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.offering.label Semantics.Templates.NounPhrase.thisSpell)),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
            (Semantics.Definitions.cast Semantics.Templates.NounPhrase.this_) (Option.some Semantics.Templates.Timing.asInstant))
          (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.offering.label Semantics.Templates.NounPhrase.this_))]
}
