/-- "Cycling [cost]" with its definition, an activated ability: "[Cost], Discard this card: Draw a
card." [CR#702.29a] -/
keyword cycling (cost : Cost) : Ability := {
  lexical := {
    lemma := "cycling",
    parameters := [[Cost]],
    bound := { surface := "cycling", stems := [Subtype, CardType, Supertype] },
    verbs := [{ lemma := "cycle", frames := [Monotransitive] }]
  },
  spelling := "cycling <cost>",
  body :=
    [activated
        (compound [cost, perform (discard this_)])
        (draw 1)]
}
