/-- A land with the basic land subtype Island has the intrinsic ability "{T}: Add {U}." [CR#305.6] -/
subtype island := {
  lexical := { lemma := "Island" },
  spelling := "Island",
  kind := .land,
  name := "Island",
  confers := []
}
