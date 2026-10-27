/-- "that <word>", e.g. `that .player`. -/
primitive that (w : NounWord) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: that <word>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.NounPhrase.pro (Semantics.Reach.word w) Semantics.Plurality.one
}
