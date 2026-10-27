/-- "named <name>" -/
primitive named (name : NameSource) : Predicate := {
  spelling := "named <name>",
  body :=
    Semantics.Primitives.Predicate.named name
}
