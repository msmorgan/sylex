primitive slice (end_ : PileEnd) (amount : Amount) (pile : Location) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.slice end_ amount pile
}
