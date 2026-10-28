/-- "Kicker [cost]" means "You may pay an additional [cost] as you cast this spell." [CR#702.33a] -/
keyword kicker (cost : Cost) : Ability := {
  lexical := {
    lemma := "kicker",
    parameters := [[Cost], [DashNonManaCost]],
    nouns := ["kicker"],
    variants := [{ lemma := "multikicker", parameters := [[Cost]] }],
    verbs := [{ lemma := "kick", frames := [Monotransitive] }]
  },
  spelling := "kicker <cost>",
  body :=
    [static (addedCost cost true once)]
}
