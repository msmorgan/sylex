template thisPlaneswalker : NounPhrase := {
  spelling := "this planeswalker",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.type Semantics.CardType.planeswalker) Semantics.Templates.NounPhrase.this_
}
