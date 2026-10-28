/-- Doctor is a creature subtype [CR#205.3m]; Doctor's companion requires its other commander to be
a legendary Time Lord Doctor creature card with no other creature types [CR#702.124m]. -/
subtype doctor := {
  lexical := { lemma := "Doctor" },
  spelling := "Doctor",
  kind := .creature,
  name := "Doctor",
  confers := []
}
