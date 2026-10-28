/-- "[cost]: <action>. Only <who> may activate this ability." -/
template activatedBy (cost : Cost) (action : Action) (who : NounPhrase) : Ability := {
  -- TODO(spelling): Context-dependent wording: [cost]: <action>. Only <who> may activate this ability.; string substitution cannot express its omitted or inflected parts.
  body := activated cost action none none none (some who)
}
