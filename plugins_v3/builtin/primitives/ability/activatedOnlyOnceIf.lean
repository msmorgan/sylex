/-- "[cost]: <action>. Activate only once <limit> and only if <guard>." -/
primitive activatedOnlyOnceIf (cost : Cost) (action : Action) (limit : UsageLimit)
    (guard : Condition) : Ability := {
  -- TODO(spelling): Context-dependent wording: [cost]: <action>. Activate only once <limit> and only if <guard>.; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Ability.activated cost action Option.none (Option.some limit) (Option.some guard) Option.none
}
