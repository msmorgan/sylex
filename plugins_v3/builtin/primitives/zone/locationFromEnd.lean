primitive locationFromEnd (end_ : Semantics.PileEnd) (ordinal : optParam Semantics.Ordinal (Semantics.Ordinal.nth 1)) (loc : Semantics.Location) : Semantics.Location := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Location.fromEnd end_ ordinal loc
}
