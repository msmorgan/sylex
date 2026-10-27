/-- "the bottom of <loc>" -/
primitive bottomOf (loc : Location) : Location := {
  spelling := "the bottom of <loc>",
  body :=
    Semantics.Primitives.Location.fromEnd Semantics.PileEnd.bottom (Semantics.Ordinal.nth 1) loc
}
