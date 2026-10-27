/-- "<performer> mills <amount> cards": from the performer's own library (`Definition`,
[CR#701.17a]). -/
action mill (amount : Amount) : Action := {
  -- TODO(spelling): mill a card / mill <amount> cards; string substitution cannot select the
  -- article and number agreement.
  lexical := { verbs := [{ lemma := "mill", frames := [Monotransitive] }] },
  body :=
    Semantics.Templates.Action.actionMove
      (Semantics.Templates.NounPhrase.slice Semantics.PileEnd.top amount (Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.actor))
      (Semantics.Templates.Location.zone Semantics.Zone.library Option.none) (Option.some Semantics.Templates.Location.graveyard)
      Semantics.ObservationPoint.arrival []
}
