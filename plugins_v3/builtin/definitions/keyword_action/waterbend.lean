/-- "<performer> waterbends <cost>" (`Definition`, [CR#701.67a,701.67b]). -/
action waterbend (cost : Cost) : Action := {
  lexical := { verbs := [{ lemma := "waterbend", past := "waterbent", frames := [Cost, Intransitive] }] },
  spelling := "waterbend <cost>",
  body :=
    Semantics.Templates.Action.actionPay
      (Semantics.Templates.Cost.withSubstitute cost
        (Semantics.Templates.ManaSubstitute.substitute
          (Semantics.Definitions.tap
            (Semantics.Templates.NounPhrase.a
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isUntapped,
                  Semantics.Templates.Predicate.or [Semantics.Templates.Predicate.isArtifact, Semantics.Templates.Predicate.isCreature],
                  Semantics.Templates.Predicate.actorControls])))
          [Semantics.Templates.PipPaid.generic]))
      Semantics.PayTimes.once Semantics.PaymentOutcome.paid Option.none Option.none
}
