/-- "Exchange <left> and <right>", two whole zones [CR#701.12f]: the cards in
them are exchanged even if one of them is empty. All or nothing, like
every exchange [CR#701.12a]. -/
action exchangeZones (left right : Location) : Action := {
  lexical := { verbs := [{ lemma := "exchange", frames := [Monotransitive, (TransitiveWithPP ["with"])] }] },
  deed := "exchange",
  spelling := "exchange <left> and <right>",
  body :=
    exchange (zones left right)
}
