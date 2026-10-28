/-- "Partner" means "You may designate two legendary cards as your commander rather than one if each
of them has partner." [CR#702.124h] -/
keyword partner : Ability := {
  lexical := {
    lemma := "partner",
    parameters := [[], [Name (Some ("with"))]],
    variants := [
                  { lemma := "partner—Character select" },
                  { lemma := "partner—Father & son" },
                  { lemma := "partner—Friends forever" },
                  { lemma := "partner—Survivors" },
                  { lemma := "choose a Background" },
                  { lemma := "doctor's companion" }
                ]
  },
  spelling := "partner",
  body :=
    []
}
