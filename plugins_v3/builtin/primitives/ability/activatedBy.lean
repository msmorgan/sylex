/-- "[cost]: <action>. Only <who> may activate this ability." -/
primitive activatedBy (cost : Cost) (action : Action) (who : NounPhrase) : Ability := {
  -- TODO(spelling): Context-dependent wording: [cost]: <action>. Only <who> may activate this ability.; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Ability.activated cost action Option.none Option.none Option.none (Option.some who)
}
