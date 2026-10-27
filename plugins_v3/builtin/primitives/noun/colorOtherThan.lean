/-- "a color other than <color>" -/
primitive colorOtherThan (color : Color) : ChoiceDomain := {
  spelling := "a color other than <color>",
  body :=
    Semantics.Primitives.ChoiceDomain.colorOtherThan color
}
