/-- A player's devotion to one or two colors. -/
primitive devotion (player : NounPhrase) (color : ColorTerm)
    (second : Option ColorTerm := none) : Amount := {
  body :=
    Semantics.Primitives.Amount.devotion player color second
}
