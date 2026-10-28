/-- "that player or planeswalker": the joined phrase just named. -/
template thatJoin : NounPhrase := {
  spelling := "that player or planeswalker",
  body :=
    that Semantics.NounWord.join
}
