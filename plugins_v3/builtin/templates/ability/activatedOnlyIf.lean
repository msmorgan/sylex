/-- "[cost]: <action>. Activate only if <guard>." -/
template activatedOnlyIf (cost : Cost) (action : Action) (guard : Condition) :
    Ability := {
  -- TODO(spelling): Context-dependent wording: [cost]: <action>. Activate only if <guard>.; string substitution cannot express its omitted or inflected parts.
  body := activated cost action none none (some guard) none
}
