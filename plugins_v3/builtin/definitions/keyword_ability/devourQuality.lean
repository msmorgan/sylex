/-- "Devour [quality] N", a variant of devour, so it carries devour's label [CR#702.82c]. -/
keyword devourQuality (quality : Predicate) (amount : Amount) : Ability := {
  lexical := {
    lemma := "devour",
    parameters := [[Amount], [Quality (None), Amount]],
    verbs := [{ lemma := "devour", frames := [Monotransitive] }]
  },
  label := "devour",
  spelling := "devour <quality> <amount>",
  body :=
    [devourEntry quality isPermanent amount]
}
