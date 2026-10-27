/-- "on top of its owner's library" -/
template onTop : Location := {
  spelling := "on top of its owner's library",
  body :=
    Semantics.Templates.Location.locationFromEnd Semantics.PileEnd.top (Semantics.Ordinal.nth 1)
      (Semantics.Templates.Location.zone Semantics.Zone.library Option.none)
}
