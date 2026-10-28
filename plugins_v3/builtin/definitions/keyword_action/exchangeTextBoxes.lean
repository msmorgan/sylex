action exchangeTextBoxes (left right : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "exchange", frames := [Monotransitive, (TransitiveWithPP ["with"])] }] },
  deed := "exchange",
  spelling := "exchange the text boxes of <left> and <right>",
  body :=
    establish (textChange left (exchangeWith right)) none
}
