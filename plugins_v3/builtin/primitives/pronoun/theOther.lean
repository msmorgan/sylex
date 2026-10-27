/-- "the other [one]", "the other [pile]": the one remaining after a choice among two. -/
primitive theOther : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: the other [one]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.NounPhrase.theRest Semantics.Plurality.one
}
