/-- "<performer> discovers N" (`Definition`, [CR#701.57a]). -/
action discover (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "discover", frames := [Amount, Intransitive] }] },
  spelling := "discover <amount>",
  body :=
    have fits :=
      Semantics.Templates.Predicate.predicateCompare [Semantics.ProjAxis.stat Semantics.Stat.manaValue] Semantics.Comparator.atMost amount;
    Semantics.Templates.Action.sequentially
      [Semantics.Definitions.exile
          (Semantics.Templates.NounPhrase.run Semantics.PileEnd.top
            (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.non Semantics.Templates.Predicate.isLand, fits])
            (Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.actor)),
        Semantics.Templates.Action.withContinuation Semantics.ContinuationPolicy.optional
          (Semantics.Definitions.castWithoutPaying (Semantics.Templates.NounPhrase.that Semantics.NounWord.card) (Option.some fits))
          Option.none
          (Option.some
            (Semantics.Templates.Action.putInto (Semantics.Templates.NounPhrase.that Semantics.NounWord.card) Semantics.Templates.Location.hand)),
        Semantics.Templates.Action.actionMove Semantics.Templates.NounPhrase.theRest Semantics.Templates.Location.wherever
          (Option.some Semantics.Templates.Location.onBottom) Semantics.ObservationPoint.arrival
          [Semantics.Templates.MoveRider.inOrder Semantics.Arrangement.randomOrder]]
}
