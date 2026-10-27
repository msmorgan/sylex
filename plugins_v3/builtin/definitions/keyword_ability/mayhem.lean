keyword mayhem (cost : Cost) : Ability := {
  lexical := { lemma := "mayhem", parameters := [[Cost], []] },
  spelling := "mayhem <cost>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
            (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisCard (Option.some Semantics.Templates.Location.yourGraveyard)
              (Semantics.Templates.PlayPayment.payingInstead cost)))
          (Semantics.Templates.Condition.did (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
            (Semantics.Definitions.discard Semantics.Templates.NounPhrase.thisCard)
            (Semantics.Templates.LookbackSpan.within Semantics.Templates.Duration.soFarThisTurn)))]
}
