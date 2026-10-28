/-- Cases have a "To solve" triggered ability and a "Solved" ability that applies once the permanent
is solved. [CR#719.3..719.3c]

No supported Oracle rules-text or Comprehensive Rules plural is attested. -/
subtype case := {
  lexical := { lemma := "Case" },
  spelling := "Case",
  kind := enchantment,
  name := "Case",
  confers := []
}
