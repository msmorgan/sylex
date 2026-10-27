/-- "untap <subject> during [<whose>] untap step" -/
template untapDuring (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec := {
  -- TODO(spelling): untap <subject> during [<whose>] untap step; string substitution cannot branch
  -- on optional fields or supply contextual agreement and morphology.
  body :=
    duringUntapStep whose (must any (untap subject))
}
