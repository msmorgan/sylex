/-- `subject` named by a Head Noun: "<subject> as a <head>". -/
primitive headed (head : NounWord) (subject : NounPhrase) : NounPhrase := {
  spelling := "<subject> as a <head>",
  body :=
    Semantics.Primitives.NounPhrase.headed head subject
}
