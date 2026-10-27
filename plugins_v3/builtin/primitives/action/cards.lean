/-- "<cards>", revealed or looked at -/
primitive cards (cards : NounPhrase) : Exposed := {
  spelling := "<cards>",
  body :=
    Semantics.Primitives.Exposed.cards cards
}
