keyword increment : Ability := {
  lexical := { lemma := "increment" },
  spelling := "increment",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isSpell)))
        (Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
          Semantics.Templates.NounPhrase.thisCreature)
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.and
            [Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent Semantics.Templates.Predicate.isCreature,
              Semantics.Templates.Condition.or
                [Semantics.Templates.Condition.compareAmt
                    (Semantics.Templates.Amount.manaValueSpentToCast (Semantics.Templates.NounPhrase.that Semantics.NounWord.spell))
                    Semantics.Comparator.greater (Semantics.Templates.Amount.powerOf Semantics.Templates.NounPhrase.thisCreature),
                  Semantics.Templates.Condition.compareAmt
                    (Semantics.Templates.Amount.manaValueSpentToCast (Semantics.Templates.NounPhrase.that Semantics.NounWord.spell))
                    Semantics.Comparator.greater (Semantics.Templates.Amount.toughnessOf Semantics.Templates.NounPhrase.thisCreature)]]))]
}
