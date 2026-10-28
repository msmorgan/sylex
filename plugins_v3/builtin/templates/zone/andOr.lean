/-- "<loc> and/or <loc>", as "library and/or graveyard". -/
template andOr (locs : List Location) : Location := {
  -- TODO(spelling): Context-dependent wording: <loc> and/or <loc>; string substitution cannot express its omitted or inflected parts.
  body := coord .andOr locs
}
