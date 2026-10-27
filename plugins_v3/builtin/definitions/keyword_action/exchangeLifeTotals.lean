action exchangeLifeTotals (parties : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "exchange", frames := [Monotransitive, (TransitiveWithPP ["with"])] }] },
  deed := "exchange",
  -- TODO(spelling): exchange the life totals of <parties>; the participant group needs
  -- coordinated possessives or a with-phrase, which string substitution cannot select.
  body :=
    Semantics.Templates.Action.actionExchange (Semantics.Templates.Exchanged.exchangedLifeTotals parties)
}
