keyword soulshift (amount : Amount) : Ability := {
  lexical := { lemma := "soulshift", parameters := [[Amount]] },
  spelling := "soulshift <amount>",
  body :=
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent) Semantics.Templates.Action.die)
        (Semantics.Templates.Action.may
          (Semantics.Templates.Action.returnToHand
            (Semantics.Templates.NounPhrase.target
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isCard, Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.spirit,
                  Semantics.Templates.Predicate.hasManaValue Semantics.Comparator.atMost amount,
                  Semantics.Templates.Predicate.in_ Semantics.Templates.Location.yourGraveyard]))
            Semantics.Templates.Location.yourGraveyard))]
}
