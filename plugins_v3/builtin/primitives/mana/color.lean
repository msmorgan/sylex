/-- "<color>" as a color term, as "devotion to black" -/
primitive color (color : Color) : ColorTerm := {
  spelling := "<color>",
  body :=
    Semantics.Primitives.ColorTerm.lit color
}
