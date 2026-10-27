keyword teamwork (amount : Amount) : Ability := {
  lexical := { lemma := "teamwork", parameters := [[Amount]] },
  spelling := "teamwork <amount>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.addedCost
          (Semantics.Templates.Cost.perform
            (Semantics.Definitions.tap
              (Semantics.Templates.NounPhrase.withTotal (Semantics.ProjAxis.stat Semantics.Stat.power) Semantics.Comparator.atLeast
                amount
                (Semantics.Templates.NounPhrase.allOf
                  (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.youControl])))))
          Bool.true Semantics.PayTimes.once)]
}
