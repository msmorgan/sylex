action exchangeValues (left right : Amount) : Action := {
  lexical := { verbs := [{ lemma := "exchange", frames := [Monotransitive, (TransitiveWithPP ["with"])] }] },
  deed := "exchange",
  spelling := "exchange <left> and <right>",
  body :=
    .exchange (.values left right)
}
