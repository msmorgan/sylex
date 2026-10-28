/-- "shuffle <subject> into its owner's library": the move, then the shuffle of that library,
which happens even if nothing moved (`Definition`, [CR#701.24c,701.24d]). -/
action shuffleInto (subject : NounPhrase) : Action := {
  -- TODO(spelling): shuffle <subject> into its/their owner's library; string substitution cannot
  -- choose agreeing possessives.
  lexical := { verbs := [{ lemma := "shuffle", frames := [Intransitive, Monotransitive, (TransitiveWithPP ["into"])] }] },
  deed := "shuffle",
  body :=
    sequentially
      [actionMove subject wherever
          (some (zone Semantics.Zone.library none)) Semantics.ObservationPoint.arrival [],
        actionShuffle (zone Semantics.Zone.library none)]
}
