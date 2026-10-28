/-- The actor of the current instruction, without binding a repeated mention. -/
primitive actor : NounPhrase := {
  -- TODO(spelling): Recover the Actor from the containing instruction before choosing its noun
  -- phrase or pronoun.
  body := Semantics.Primitives.NounPhrase.actor
}
