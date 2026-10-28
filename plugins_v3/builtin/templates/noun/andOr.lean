/-- "<np> and/or <np>", as "a card named Magnifying Glass and/or a card named Thinking Cap". -/
template andOr (phrases : List NounPhrase := []) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: <np> and/or <np>; string substitution cannot express its omitted or inflected parts.
  body := coord .andOr phrases
}
