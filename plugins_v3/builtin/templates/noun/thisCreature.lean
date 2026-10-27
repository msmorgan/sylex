template thisCreature : NounPhrase := {
  spelling := "this creature",
  body :=
    .headed (.type Semantics.CardType.creature) .this
}
