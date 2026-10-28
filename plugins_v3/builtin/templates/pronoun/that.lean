/-- "that <word>", e.g. `that .player`. -/
template that (w : NounWord) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: that <word>; string substitution cannot express its omitted or inflected parts.
  body := pro (.word w) .one
}
