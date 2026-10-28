/-- "<performer> collects evidence N" (`Definition`, [CR#701.59a]). -/
action collectEvidence (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "collect", frames := [TransitiveAmount, Monotransitive] }] },
  spelling := "collect evidence <amount>",
  body :=
    exile
      (withTotal (.stat .manaValue) .atLeast amount
        (allOf
          (and
            [isCard,
              in_ (graveyardOf actor)])))
}
