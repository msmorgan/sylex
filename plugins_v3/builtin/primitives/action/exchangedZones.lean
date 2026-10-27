primitive exchangedZones (left : Semantics.Location) (right : Semantics.Location) : Semantics.Exchanged := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Exchanged.zones left right
}
