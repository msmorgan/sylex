/-- "on the bottom of its owner's library" -/
template onBottom : Location := {
  spelling := "on the bottom of its owner's library",
  body :=
    fromEnd .bottom (.nth 1)
      (zone .library none)
}
