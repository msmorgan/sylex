/-- "<np> <verb>s": the clause's subject, in the active voice. -/
primitive actor (np : NounPhrase) : Subject := {
  body :=
    Semantics.Primitives.Subject.actor np
}
