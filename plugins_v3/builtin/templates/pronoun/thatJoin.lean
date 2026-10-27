/-- "that player or planeswalker": the joined phrase just named. -/
template thatJoin : NounPhrase := {
  spelling := "that player or planeswalker",
  body :=
    Semantics.Templates.NounPhrase.that Semantics.NounWord.join
}
