/-- "[you may have it enter] as a copy of <source> [except …]" -/
primitive asCopyOf (optional : Bool) (source : NounPhrase) (exceptions : List CopyExcept) :
    MoveRider := {
  -- TODO(spelling): Context-dependent wording: [you may have it enter] as a copy of <source> [except …]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.MoveRider.asCopyOf optional source exceptions
}
