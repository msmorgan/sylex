/-- "on top of its owner's library" -/
template onTop : Location := {
  spelling := "on top of its owner's library",
  body :=
    locationFromEnd Semantics.PileEnd.top (Semantics.Ordinal.nth 1)
      (zone Semantics.Zone.library none)
}
