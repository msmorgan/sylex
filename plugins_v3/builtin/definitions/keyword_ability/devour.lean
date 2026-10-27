/-- "Devour N": devour over creatures [CR#702.82a]. -/
keyword devour (amount : Amount) : Ability := {
  lexical := {
    lemma := "devour",
    parameters := [[Amount], [Quality (None), Amount]],
    verbs := [{ lemma := "devour", frames := [Monotransitive] }]
  },
  spelling := "devour <amount>",
  body :=
    [devourEntry isCreature isCreature amount]
}
