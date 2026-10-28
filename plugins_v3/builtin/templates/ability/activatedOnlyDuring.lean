/-- "[cost]: <action>. Activate only <timing>." -/
template activatedOnlyDuring (cost : Cost) (action : Action) (timing : Timing) :
    Ability := {
  -- TODO(spelling): Context-dependent wording: [cost]: <action>. Activate only <timing>.; string substitution cannot express its omitted or inflected parts.
  body := activated cost action (some timing) none none none
}
