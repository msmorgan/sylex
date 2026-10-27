template thisLand : NounPhrase := {
  spelling := "this land",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.type Semantics.CardType.land) Semantics.Templates.NounPhrase.this_
}
