/-- "<spec> during [<whose>] untap step" -/
template duringUntapStep (whose : Option NounPhrase) (spec : StaticSpec) : StaticSpec := {
  -- TODO(spelling): Context-dependent wording: <spec> during [<whose>] untap step; string substitution cannot express its omitted or inflected parts.
  body :=
    partScope Semantics.TurnPart.untapStep whose spec
}
