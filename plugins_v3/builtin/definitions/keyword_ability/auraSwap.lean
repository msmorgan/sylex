keyword auraSwap (cost : Cost) : Ability := {
  lexical := { lemma := "Aura swap", parameters := [[Cost]] },
  spelling := "aura swap <cost>",
  body :=
    [Semantics.Templates.Ability.activated cost
        (Semantics.Templates.Action.may
          (Semantics.Definitions.exchangeCards Semantics.Templates.NounPhrase.thisPermanent
            (Semantics.Templates.NounPhrase.a
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isCard, Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.aura,
                  Semantics.Templates.Predicate.in_ Semantics.Templates.Location.yourHand]))))]
}
