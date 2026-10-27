keyword transfigure (cost : Cost) : Ability := {
  lexical := { lemma := "transfigure", parameters := [[Cost]] },
  spelling := "transfigure <cost>",
  body :=
    [Semantics.Templates.Ability.activatedOnlyDuring
        (Semantics.Templates.Cost.compound
          [cost, Semantics.Templates.Cost.perform (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.thisPermanent)])
        (Semantics.Templates.Action.sequentially
          [Semantics.Definitions.search Semantics.Templates.Location.yourLibrary
              (Semantics.Templates.NounPhrase.a
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.isCard, Semantics.Templates.Predicate.isCreature,
                    Semantics.Templates.Predicate.hasManaValue Semantics.Comparator.eq
                      (Semantics.Templates.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue)
                        Semantics.Templates.NounPhrase.thisPermanent)])),
            Semantics.Templates.Action.putOntoBattlefield (Semantics.Templates.NounPhrase.that Semantics.NounWord.card),
            Semantics.Definitions.shuffle])
        Semantics.Templates.Timing.asSorcery]
}
