/-- "<parties> exchange life totals" [CR#701.12c]: each player gains or loses
the life needed to equal the other's previous life total. All or nothing,
like every exchange [CR#701.12a]. -/
action exchangeLifeTotals (parties : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "exchange", frames := [Monotransitive, (TransitiveWithPP ["with"])] }] },
  deed := "exchange",
  -- TODO(spelling): exchange the life totals of <parties>; the participant group needs
  -- coordinated possessives or a with-phrase, which string substitution cannot select.
  body :=
    exchange (lifeTotals parties)
}
