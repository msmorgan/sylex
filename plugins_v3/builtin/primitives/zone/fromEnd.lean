primitive fromEnd (end_ : PileEnd) (ordinal : Ordinal := .nth 1) (loc : Location) : Location := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Location.fromEnd end_ ordinal loc
}
