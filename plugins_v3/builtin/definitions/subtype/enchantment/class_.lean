/-- A Class level bar represents an activated ability that raises its level and a static ability
that grants abilities at that level or greater. Class levels are separate from level counters.
[CR#716.2a,716.4]

No supported Oracle rules-text or Comprehensive Rules plural is attested. -/
subtype class_ := {
  lexical := { lemma := "Class" },
  spelling := "Class",
  kind := .enchantment,
  name := "Class",
  confers := []
}
