/-- "Choose one — <first>; or <second>." -/
template chooseOne (first second : Action) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    actionChooseModes (exactly 1)
      [(none, none, first), (none, none, second)]
}
