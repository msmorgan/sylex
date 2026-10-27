keyword ascend : Ability := {
  lexical := { lemma := "ascend" },
  spelling := "ascend",
  body :=
    [Semantics.Templates.Ability.byCardClass
        (Semantics.Templates.Ability.static
          (Semantics.Templates.StaticSpec.whileHolds
            (Semantics.Templates.Condition.and
              [Semantics.Templates.Condition.compareAmt
                  (Semantics.Templates.Amount.countOf
                    (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isPermanent, Semantics.Templates.Predicate.youControl]))
                  Semantics.Comparator.atLeast 10,
                Semantics.Templates.Condition.not
                  (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.you
                    (Semantics.Templates.Predicate.hasDesignation Semantics.Definitions.citysBlessing))])
            (Semantics.Templates.Action.gainDesignation Semantics.Templates.NounPhrase.you Semantics.Definitions.citysBlessing
              (Option.some Semantics.Templates.Duration.restOfGame))))
        (Semantics.Templates.Ability.spell
          (Semantics.Templates.Action.doIf
            (Semantics.Templates.Condition.and
              [Semantics.Templates.Condition.compareAmt
                  (Semantics.Templates.Amount.countOf
                    (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isPermanent, Semantics.Templates.Predicate.youControl]))
                  Semantics.Comparator.atLeast 10,
                Semantics.Templates.Condition.not
                  (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.you
                    (Semantics.Templates.Predicate.hasDesignation Semantics.Definitions.citysBlessing))])
            (Semantics.Templates.Action.gainDesignation Semantics.Templates.NounPhrase.you Semantics.Definitions.citysBlessing
              (Option.some Semantics.Templates.Duration.restOfGame))))]
}
