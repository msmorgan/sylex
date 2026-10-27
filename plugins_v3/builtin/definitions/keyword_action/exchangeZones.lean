action exchangeZones (left right : Location) : Action := {
  lexical := { verbs := [{ lemma := "exchange", frames := [Monotransitive, (TransitiveWithPP ["with"])] }] },
  deed := "exchange",
  spelling := "exchange <left> and <right>",
  body :=
    .exchange (.zones left right)
}
