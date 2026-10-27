/-- "<np> <verb>s": the clause's subject, in the active voice. -/
primitive actor (np : NounPhrase) : Subject := {
  -- TODO(spelling): Context-dependent wording: <np> <verb>s; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Subject.actor np
}
