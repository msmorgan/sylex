/-- "… by <np>": the clause's subject, in the passive voice. -/
primitive by_ (np : NounPhrase) : Subject := {
  spelling := "by <np>",
  body :=
    Semantics.Primitives.Subject.by np
}
