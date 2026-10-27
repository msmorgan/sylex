/-- "<performer> collects evidence N" (`Definition`, [CR#701.59a]). -/
action collectEvidence (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "collect", frames := [TransitiveAmount, Monotransitive] }] },
  spelling := "collect evidence <amount>",
  body :=
    Semantics.Definitions.exile
      (Semantics.Templates.NounPhrase.withTotal (Semantics.ProjAxis.stat Semantics.Stat.manaValue) Semantics.Comparator.atLeast amount
        (Semantics.Templates.NounPhrase.allOf
          (Semantics.Templates.Predicate.and
            [Semantics.Templates.Predicate.isCard,
              Semantics.Templates.Predicate.in_ (Semantics.Templates.Location.graveyardOf Semantics.Templates.NounPhrase.actor)])))
}
