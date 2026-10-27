template thisCreature : NounPhrase := {
  spelling := "this creature",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.type Semantics.CardType.creature) Semantics.Templates.NounPhrase.this_
}
