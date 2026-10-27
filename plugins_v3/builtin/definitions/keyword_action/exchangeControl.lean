action exchangeControl (left right : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "exchange", frames := [Monotransitive, (TransitiveWithPP ["with"])] }] },
  deed := "exchange",
  spelling := "exchange control of <left> and <right>",
  body :=
    .exchange
        (.controlOf left
          right)
}
