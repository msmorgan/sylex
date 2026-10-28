/-- "<loc> or <loc>", as "on the top or bottom of your library". -/
primitive or (locs : List Location) : Location := {
  -- TODO(spelling): Context-dependent wording: <loc> or <loc>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Location.coord Semantics.Coordinator.or locs
}
