keyword bargain : Ability := {
  lexical := { lemma := "bargain", verbs := [{ lemma := "bargain", frames := [Monotransitive] }] },
  spelling := "bargain",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.addedCost
          (Semantics.Templates.Cost.perform
            (Semantics.Definitions.sacrifice
              (Semantics.Templates.NounPhrase.a
                (Semantics.Templates.Predicate.or
                  [Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isArtifact, Semantics.Templates.Predicate.isPermanent],
                    Semantics.Templates.Predicate.and
                      [Semantics.Templates.Predicate.isEnchantment, Semantics.Templates.Predicate.isPermanent],
                    Semantics.Templates.Predicate.isToken]))))
          Bool.true Semantics.PayTimes.once)]
}
