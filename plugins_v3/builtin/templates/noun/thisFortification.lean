template thisFortification : NounPhrase := {
  spelling := "this Fortification",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.subtype Semantics.Definitions.fortification) Semantics.Templates.NounPhrase.this_
}
