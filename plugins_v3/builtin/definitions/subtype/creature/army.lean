/-- Army, named apart from the `army` predicate template (an Army the performer controls). -/
subtype army := {
  lexical := { lemma := "Army" },
  spelling := "Army",
  kind := .creature,
  name := "Army",
  confers := []
}
