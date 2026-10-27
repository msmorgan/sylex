keyword transmute (cost : Cost) : Ability := {
  lexical := { lemma := "transmute", parameters := [[Cost]] },
  spelling := "transmute <cost>",
  body :=
    [Semantics.Templates.Ability.activatedOnlyDuring
        (Semantics.Templates.Cost.compound [cost, Semantics.Templates.Cost.perform (Semantics.Definitions.discard Semantics.Templates.NounPhrase.this_)])
        (Semantics.Templates.Action.sequentially
          [Semantics.Definitions.search Semantics.Templates.Location.yourLibrary
              (Semantics.Templates.NounPhrase.a
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.isCard,
                    Semantics.Templates.Predicate.hasManaValue Semantics.Comparator.eq
                      (Semantics.Templates.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue)
                        (Semantics.Templates.NounPhrase.the
                          (Semantics.Templates.Predicate.and
                            [Semantics.Templates.Predicate.was (Semantics.Definitions.discard Semantics.Templates.NounPhrase.which),
                              Semantics.Templates.Predicate.isCard])))])),
            Semantics.Definitions.reveal (Semantics.Templates.NounPhrase.that Semantics.NounWord.card),
            Semantics.Templates.Action.putInto Semantics.Templates.NounPhrase.it Semantics.Templates.Location.hand, Semantics.Definitions.shuffle])
        Semantics.Templates.Timing.asSorcery]
}
