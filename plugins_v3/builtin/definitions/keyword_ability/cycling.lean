/-- "Cycling [cost]" with its definition, an activated ability: "[Cost], Discard this card: Draw a
card." [CR#702.29a] -/
keyword cycling (cost : Cost) : Ability := {
  lexical := {
    lemma := "cycling",
    parameters := [[Cost], [DashNonManaCost]],
    bound := { surface := "cycling", stems := [Subtype, CardType, Supertype] },
    verbs := [{ lemma := "cycle", frames := [Monotransitive] }]
  },
  -- TODO(spelling): cycling [mana cost] / cycling—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [Semantics.Templates.Ability.activated
        (Semantics.Templates.Cost.compound [cost, Semantics.Templates.Cost.perform (Semantics.Definitions.discard Semantics.Templates.NounPhrase.this_)])
        (Semantics.Templates.Action.draw 1)]
}
