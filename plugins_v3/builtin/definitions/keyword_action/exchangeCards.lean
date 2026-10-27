action exchangeCards (left right : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "exchange", frames := [Monotransitive, (TransitiveWithPP ["with"])] }] },
  deed := "exchange",
  spelling := "exchange <left> and <right>",
  body :=
    Semantics.Templates.Action.actionExchange (Semantics.Templates.Exchanged.exchangedCardsAcross left right)
}
