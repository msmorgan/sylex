/-- "shuffle <subject> into its owner's library": the move, then the shuffle of that library,
which happens even if nothing moved (`Definition`, [CR#701.24c,701.24d]). -/
action shuffleInto (subject : NounPhrase) : Action := {
  -- TODO(spelling): shuffle <subject> into its/their owner's library; string substitution cannot
  -- choose agreeing possessives.
  lexical := { verbs := [{ lemma := "shuffle", frames := [Intransitive, Monotransitive, (TransitiveWithPP ["into"])] }] },
  deed := "shuffle",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.actionMove subject Semantics.Templates.Location.wherever
          (Option.some (Semantics.Templates.Location.zone Semantics.Zone.library Option.none)) Semantics.ObservationPoint.arrival [],
        Semantics.Templates.Action.actionShuffle (Semantics.Templates.Location.zone Semantics.Zone.library Option.none)]
}
