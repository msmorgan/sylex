template anOpponent : NounPhrase := {
  spelling := "an opponent",
  body :=
    Semantics.Templates.NounPhrase.a (Semantics.Templates.Predicate.isOpponentOf Semantics.Templates.NounPhrase.you)
}
