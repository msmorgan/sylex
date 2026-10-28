/-- "<loc> and/or <loc>", as "library and/or graveyard". -/
primitive andOr (locs : List Location) : Location := {
  -- TODO(spelling): Context-dependent wording: <loc> and/or <loc>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Location.coord Semantics.Coordinator.andOr locs
}
