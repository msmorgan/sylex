/-- "anywhere" -/
primitive wherever : Location := {
  spelling := "anywhere",
  body :=
    Semantics.Primitives.Location.anyLocation
}
