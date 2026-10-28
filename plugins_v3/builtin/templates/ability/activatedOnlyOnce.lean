/-- "[cost]: <action>. Activate only once <limit>." -/
template activatedOnlyOnce (cost : Cost) (action : Action) (limit : UsageLimit) :
    Ability := {
  -- TODO(spelling): Context-dependent wording: [cost]: <action>. Activate only once <limit>.; string substitution cannot express its omitted or inflected parts.
  body := activated cost action none (some limit) none none
}
