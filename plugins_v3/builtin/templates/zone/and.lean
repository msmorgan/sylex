/-- "<loc> and <loc>", as "graveyard, hand, and library". -/
template and (locs : List Location) : Location := {
  -- TODO(spelling): Context-dependent wording: <loc> and <loc>; string substitution cannot express its omitted or inflected parts.
  body := coord .and locs
}
