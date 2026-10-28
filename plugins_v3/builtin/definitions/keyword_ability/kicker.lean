keyword kicker (cost : Cost) : Ability := {
  lexical := {
    lemma := "kicker",
    parameters := [[Cost], [DashNonManaCost]],
    nouns := ["kicker"],
    variants := [{ lemma := "multikicker", parameters := [[Cost]] }],
    verbs := [{ lemma := "kick", frames := [Monotransitive] }]
  },
  -- TODO(spelling): kicker [mana cost] / kicker—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [static (addedCost cost true .once)]
}
