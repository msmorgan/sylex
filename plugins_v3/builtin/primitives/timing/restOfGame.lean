/-- "for the rest of the game" -/
primitive restOfGame : Duration := {
  spelling := "for the rest of the game",
  body :=
    Semantics.Primitives.Duration.between Option.none Option.none
}
