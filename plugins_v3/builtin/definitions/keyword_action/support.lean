/-- "support N" (`Definition`, [CR#701.41a]). -/
action support (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "support", frames := [Amount] }] },
  spelling := "support <amount>",
  body :=
    Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
      (Semantics.Templates.NounPhrase.eachOf
        (Semantics.Templates.NounPhrase.described (Semantics.Templates.DetPhrase.detPhraseTarget (Semantics.Templates.Quantity.upToOf amount))
          (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.isOther])))
}
