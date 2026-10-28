/-- The actor of the current instruction, without binding a repeated mention. -/
primitive actor : NounPhrase := {
  -- TODO(spelling): Dynamic actor reference used by expansions; its wording depends on the instruction.
  body := Semantics.Primitives.NounPhrase.actor
}
