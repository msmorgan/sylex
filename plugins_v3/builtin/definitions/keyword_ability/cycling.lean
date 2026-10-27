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
    [activated (.compound [cost, .perform (discard this_)]) (draw 1)]
}
