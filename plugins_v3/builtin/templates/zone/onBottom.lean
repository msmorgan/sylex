/-- "on the bottom of its owner's library" -/
template onBottom : Location := {
  spelling := "on the bottom of its owner's library",
  body :=
    Semantics.Templates.Location.locationFromEnd Semantics.PileEnd.bottom (Semantics.Ordinal.nth 1)
      (Semantics.Templates.Location.zone Semantics.Zone.library Option.none)
}
