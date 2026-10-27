template thisSpacecraft : NounPhrase := {
  spelling := "this Spacecraft",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.subtype Semantics.Definitions.spacecraft) Semantics.Templates.NounPhrase.this_
}
