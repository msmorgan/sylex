/-- Banding is a static ability that modifies the rules for combat. [CR#702.22a] -/
keyword banding : Ability := {
  lexical := {
    lemma := "banding",
    variants := [{ lemma := "bands with other", parameters := [[Quality (None)]] }],
    verbs := [{ lemma := "band", frames := [TransitiveWithPP (["with"])] }]
  },
  spelling := "banding",
  body :=
    []
}
