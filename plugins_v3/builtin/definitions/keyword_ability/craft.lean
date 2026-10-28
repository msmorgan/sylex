/-- "Craft with [materials] [cost]" means "[Cost], Exile this permanent, Exile [materials] from
among permanents you control and/or cards in your graveyard: Return this card to the battlefield
transformed under its owner’s control. Activate only as a sorcery." [CR#702.167a] -/
keyword craft (cost : Cost) : Ability := {
  lexical := {
    lemma := "craft",
    parameters := [[Quality (Some ("with")), Cost]],
    verbs := [{ lemma := "craft", frames := [Monotransitive] }]
  },
  -- TODO(spelling): Craft with [materials] [cost]; the Cost combines material selection and
  -- payment, which require structural decomposition.
  body :=
    []
}
