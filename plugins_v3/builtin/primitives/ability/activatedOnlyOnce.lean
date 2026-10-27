/-- "[cost]: <action>. Activate only once <limit>." -/
primitive activatedOnlyOnce (cost : Cost) (action : Action) (limit : UsageLimit) :
    Ability := {
  -- TODO(spelling): Context-dependent wording: [cost]: <action>. Activate only once <limit>.; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Ability.activated cost action Option.none (Option.some limit) Option.none Option.none
}
