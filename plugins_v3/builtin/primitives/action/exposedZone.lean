primitive exposedZone (zone : Semantics.Location) : Semantics.Exposed := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Exposed.zone zone
}
