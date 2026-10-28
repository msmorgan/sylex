/-- "the rest [of them]" -/
primitive theRest (plur : Plurality := .many) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: the rest [of them]; string substitution cannot express its omitted or inflected parts.
  body := Semantics.Primitives.NounPhrase.theRest plur
}
