template thisPlaneswalker : NounPhrase := {
  spelling := "this planeswalker",
  body :=
    headed (Semantics.NounWord.type Semantics.CardType.planeswalker) this_
}
