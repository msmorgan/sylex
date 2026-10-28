template place (end_ : PileEnd) (pile : Location) : Location := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body := fromEnd end_ (.nth 1) pile
}
