primitive letter (letter : Letter) : Amount := {
  spelling := "<letter>",
  body :=
    Semantics.Primitives.Amount.letter letter
}
