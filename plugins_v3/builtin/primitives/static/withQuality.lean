/-- "of <quality>", as "of the chosen color" -/
primitive withQuality (quality : Predicate) : WrittenQuality := {
  spelling := "of <quality>",
  body :=
    Semantics.Primitives.WrittenQuality.chosen quality
}
