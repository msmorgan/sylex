/-- "<performer> discovers N" (`Definition`, [CR#701.57a]). -/
action discover (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "discover", frames := [Amount, Intransitive] }] },
  spelling := "discover <amount>",
  body :=
    have fits :=
      predicateCompare [Semantics.ProjAxis.stat Semantics.Stat.manaValue] Semantics.Comparator.atMost amount;
    sequentially
      [exile
          (run Semantics.PileEnd.top
            (and [non isLand, fits])
            (libraryOf actor)),
        withContinuation Semantics.ContinuationPolicy.optional
          (castWithoutPaying (that Semantics.NounWord.card) (some fits))
          none
          (some
            (putInto (that Semantics.NounWord.card) hand)),
        actionMove theRest wherever
          (some onBottom) Semantics.ObservationPoint.arrival
          [inOrder Semantics.Arrangement.randomOrder]]
}
