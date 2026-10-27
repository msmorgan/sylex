/-- "[cost]: <action>. Activate only <timing>." -/
primitive activatedOnlyDuring (cost : Cost) (action : Action) (timing : Timing) :
    Ability := {
  -- TODO(spelling): Context-dependent wording: [cost]: <action>. Activate only <timing>.; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Ability.activated cost action (Option.some timing) Option.none Option.none Option.none
}
