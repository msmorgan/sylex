/-- Junk is an artifact subtype [CR#205.3g], also used by the predefined Junk token [CR#111.10t].

No supported Oracle rules-text or Comprehensive Rules plural is attested. -/
subtype junk := {
  lexical := { lemma := "Junk" },
  spelling := "Junk",
  kind := artifact,
  name := "Junk",
  confers := []
}
