/-- "the top of <loc>": the first place from its top [CR#401.7]. -/
-- [CR#401.7]
primitive topOf (loc : Location) : Location := {
  spelling := "the top of <loc>",
  body :=
    Semantics.Primitives.Location.fromEnd Semantics.PileEnd.top (Semantics.Ordinal.nth 1) loc
}
