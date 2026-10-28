/-- "on top of its owner's library" -/
template onTop : Location := {
  spelling := "on top of its owner's library",
  body :=
    fromEnd .top (.nth 1)
      (zone .library none)
}
