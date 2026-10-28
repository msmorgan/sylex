/-- Authoring primitive for `Predicate.colorIs`. -/
primitive hasColor (c : Color) : Predicate := {
  spelling := "<c>",
  body :=
    Semantics.Primitives.Predicate.colorIs c
}
