keyword storied : Ability := {
  lexical := { lemma := "storied" },
  spelling := "storied",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.whileHolds
          (Semantics.Templates.Condition.and
            [Semantics.Templates.Condition.compareAmt
                (Semantics.Templates.Amount.countOf
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.isPermanent, Semantics.Templates.Predicate.youControl,
                      Semantics.Templates.Predicate.andOr
                        [Semantics.Templates.Predicate.isArtifact, Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.saga,
                          Semantics.Templates.Predicate.hasSupertype Semantics.Supertype.legendary]]))
                Semantics.Comparator.atLeast 3,
              Semantics.Templates.Condition.not
                (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.you
                  (Semantics.Templates.Predicate.hasDesignation Semantics.Definitions.enduringStory))])
          (Semantics.Templates.Action.gainDesignation Semantics.Templates.NounPhrase.you Semantics.Definitions.enduringStory
            (Option.some Semantics.Templates.Duration.restOfGame)))]
}
