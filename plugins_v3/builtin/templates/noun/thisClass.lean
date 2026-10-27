template thisClass : NounPhrase := {
  spelling := "this Class",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.subtype Semantics.Definitions.class_) Semantics.Templates.NounPhrase.this_
}
