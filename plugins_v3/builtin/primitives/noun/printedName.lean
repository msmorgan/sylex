/-- "a card named <name>" -/
primitive printedName (name : String) : NameSource := {
  spelling := "a card named <name>",
  body :=
    Semantics.Primitives.NameSource.printed name
}
