/-- "<performer> mills <amount> cards": from the performer's own library (`Definition`,
[CR#701.17a]). -/
action mill (amount : Amount) : Action := {
  -- TODO(spelling): mill a card / mill <amount> cards; string substitution cannot select the
  -- article and number agreement.
  lexical := { verbs := [{ lemma := "mill", frames := [Monotransitive] }] },
  body :=
    actionMove
      (slice .top amount (libraryOf actor))
      (zone .library none) (some graveyard)
      .arrival []
}
