/-- "[cost]: <action>. Activate only once <limit> and only if <guard>." -/
template activatedOnlyOnceIf (cost : Cost) (action : Action) (limit : UsageLimit)
    (guard : Condition) : Ability := {
  -- TODO(spelling): Context-dependent wording: [cost]: <action>. Activate only once <limit> and only if <guard>.; string substitution cannot express its omitted or inflected parts.
  body := activated cost action none (some limit) (some guard) none
}
