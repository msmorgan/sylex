template thisSpell : NounPhrase := {
  spelling := "this spell",
  body :=
    Semantics.Templates.NounPhrase.headed Semantics.NounWord.spell Semantics.Templates.NounPhrase.this_
}
