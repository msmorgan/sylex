keyword amplify (amount : Amount) : Ability := {
  lexical := { lemma := "amplify", parameters := [[Amount]] },
  spelling := "amplify <amount>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent)
            Semantics.Templates.Action.enter)
          (Semantics.Templates.Replacement.before
            (Semantics.Definitions.reveal
              (Semantics.Templates.NounPhrase.counted Semantics.Templates.Quantity.anyNumber
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.cardIn Semantics.Templates.Location.yourHand,
                    Semantics.Templates.Predicate.shares (Semantics.QualitySort.subtype Semantics.CardType.creature)
                      Semantics.Agreement.share Semantics.Templates.NounPhrase.thisPermanent])))
            (Option.some
              (Semantics.Templates.Replacement.alongside
                (Semantics.Templates.Action.putCounters
                  (Semantics.Templates.Amount.forEach amount
                    (Semantics.Templates.Predicate.and
                      [Semantics.Templates.Predicate.isCard,
                        Semantics.Templates.Predicate.was (Semantics.Definitions.reveal Semantics.Templates.NounPhrase.which)
                          (Option.some Semantics.Templates.LookbackSpan.thisWay)]))
                  (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
                  Semantics.Templates.NounPhrase.thisPermanent)))))]
}
