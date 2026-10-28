template thisCreature : NounPhrase := {
  spelling := "this creature",
  body :=
    headed (Semantics.NounWord.type Semantics.CardType.creature) this_
}
