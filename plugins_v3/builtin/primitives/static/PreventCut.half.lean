/-- "half that damage, rounded <rounding>" -/
primitive PreventCut.half (rounding : RoundMode) : PreventCut := {
  spelling := "half that damage, rounded <rounding>",
  body :=
    Semantics.Primitives.PreventCut.half rounding
}
