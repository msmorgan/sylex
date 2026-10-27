keyword dredge (amount : Amount) : Ability := {
  lexical := { lemma := "dredge", parameters := [[Amount]] },
  spelling := "dredge <amount>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.replace
            (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you) (Semantics.Templates.Action.draw 1))
            (Semantics.Templates.Replacement.instead
              (Semantics.Templates.Action.may
                (Semantics.Templates.Action.sequentially
                  [Semantics.Definitions.mill amount,
                    Semantics.Templates.Action.returnToHand Semantics.Templates.NounPhrase.this_ Semantics.Templates.Location.yourGraveyard]))))
          (Semantics.Templates.Condition.compareAmt
            (Semantics.Templates.Amount.countOf (Semantics.Templates.Predicate.cardIn Semantics.Templates.Location.yourLibrary))
            Semantics.Comparator.atLeast amount))]
}
