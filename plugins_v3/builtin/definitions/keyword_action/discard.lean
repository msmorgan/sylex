/-- "<performer> discards <subject>": a move from the performer's own hand, the card's owner's,
to the graveyard (`Definition`, [CR#701.9a]). -/
action discard (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "discard", frames := [Monotransitive] }] },
  spelling := "discard <subject>",
  body :=
    Semantics.Templates.Action.actionMove subject (Semantics.Templates.Location.zone Semantics.Zone.hand (Option.some Semantics.Templates.NounPhrase.actor))
      (Option.some Semantics.Templates.Location.graveyard) Semantics.ObservationPoint.arrival []
}
