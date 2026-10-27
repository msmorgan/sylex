/-- "… by <np>": the clause's subject, in the passive voice. -/
primitive by_ (np : NounPhrase) : Subject := {
  -- TODO(spelling): Context-dependent wording: … by <np>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Subject.by np
}
