/-- "[cost]: <action>. Activate only if <guard>." -/
primitive activatedOnlyIf (cost : Cost) (action : Action) (guard : Condition) :
    Ability := {
  -- TODO(spelling): Context-dependent wording: [cost]: <action>. Activate only if <guard>.; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Ability.activated cost action Option.none Option.none (Option.some guard) Option.none
}
