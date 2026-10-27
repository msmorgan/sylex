/-- "those <word>s", e.g. `those .card`. -/
primitive those (w : NounWord) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: those <word>s; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.NounPhrase.pro (Semantics.Reach.word w) Semantics.Plurality.many
}
