/-- Supported Oracle uses `Leeches` only in names and self-reference, not as a subtype noun. -/
subtype leech := {
  lexical := { lemma := "Leech" },
  spelling := "Leech",
  kind := .creature,
  name := "Leech",
  confers := []
}
