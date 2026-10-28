/-- "[you may have it enter] as a copy of <source> [except …]" -/
primitive asCopyOf (optional : Bool) (source : NounPhrase) (exceptions : List CopyExcept) :
    MoveRider := {
  -- TODO(spelling): Format and coordinate the exceptions list entries, including the empty case.
  body :=
    Semantics.Primitives.MoveRider.asCopyOf optional source exceptions
}
