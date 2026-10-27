template thisSiege : NounPhrase := {
  spelling := "this Siege",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.subtype Semantics.Definitions.siege) Semantics.Templates.NounPhrase.this_
}
