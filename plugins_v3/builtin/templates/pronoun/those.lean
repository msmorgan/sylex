/-- "those <word>s", e.g. `those .card`. -/
template those (w : NounWord) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: those <word>s; string substitution cannot express its omitted or inflected parts.
  body := pro (.word w) .many
}
