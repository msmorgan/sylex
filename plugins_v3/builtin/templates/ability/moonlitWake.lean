template moonlitWake : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := whenever (does (a isCreature) die) (gainLife 1)
}
