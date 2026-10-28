/-- "on the bottom of its owner's library" -/
template onBottom : Location := {
  spelling := "on the bottom of its owner's library",
  body :=
    locationFromEnd Semantics.PileEnd.bottom (Semantics.Ordinal.nth 1)
      (zone Semantics.Zone.library none)
}
