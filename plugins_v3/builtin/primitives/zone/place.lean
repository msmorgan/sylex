primitive place (end_ : PileEnd) (pile : Location) : Location := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Location.fromEnd end_ (Semantics.Ordinal.nth 1) pile
}
