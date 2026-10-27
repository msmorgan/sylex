primitive devotion (player : NounPhrase) (color : ColorTerm)
    (second : Option ColorTerm := none) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Amount.devotion player color second
}
