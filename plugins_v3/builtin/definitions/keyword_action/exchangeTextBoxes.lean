action exchangeTextBoxes (left right : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "exchange", frames := [Monotransitive, (TransitiveWithPP ["with"])] }] },
  deed := "exchange",
  spelling := "exchange the text boxes of <left> and <right>",
  body :=
    Semantics.Templates.Action.establish (Semantics.Templates.StaticSpec.textChange left (Semantics.Templates.TextChange.exchangeWith right)) Option.none
}
