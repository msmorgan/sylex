/-- "Exchange the text boxes of <left> and <right>" [CR#701.12h]: a Text Change
that replaces each permanent's rules text with the other's previous rules text [CR#612.5].
With no duration stated, it lasts until the end of the game [CR#611.2a]. -/
action exchangeTextBoxes (left right : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "exchange", frames := [Monotransitive, (TransitiveWithPP ["with"])] }] },
  deed := "exchange",
  spelling := "exchange the text boxes of <left> and <right>",
  body :=
    establish (textChange left (exchangeWith right)) none
}
