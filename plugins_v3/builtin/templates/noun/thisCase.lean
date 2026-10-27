template thisCase : NounPhrase := {
  spelling := "this Case",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.subtype Semantics.Definitions.case) Semantics.Templates.NounPhrase.this_
}
