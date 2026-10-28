/-- "anywhere other than <loc>" -/
primitive not (loc : Location) : Location := {
  spelling := "anywhere other than <loc>",
  body :=
    Semantics.Primitives.Location.not loc
}
