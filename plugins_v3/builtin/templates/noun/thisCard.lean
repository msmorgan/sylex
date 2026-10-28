/-- "this card": the source as a card, in whatever zone it is [CR#109.1]. -/
-- [CR#109.1]
template thisCard : NounPhrase := {
  spelling := "this card",
  body :=
    headed Semantics.NounWord.card this_
}
