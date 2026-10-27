template doublingSeasonTokens : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := static (replace
    (causedBy byAnEffect
      (would any (createObject anyAmount (token (copyOf (counted (orMore 1) isToken)) []) you)))
    (instead (create (times 2 groupSize) asThose [])))
}
