template thisRoom : NounPhrase := {
  spelling := "this Room",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.subtype Semantics.Definitions.room) Semantics.Templates.NounPhrase.this_
}
