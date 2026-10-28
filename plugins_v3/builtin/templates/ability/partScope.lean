/-- "During [<whose>] <part>, <spec>" -/
template partScope (part : TurnPart) (whose : Option NounPhrase) (spec : StaticSpec) :
    StaticSpec := {
  -- TODO(spelling): Context-dependent wording: During [<whose>] <part>, <spec>; string substitution cannot express its omitted or inflected parts.
  body :=
    conditional spec (duringPart part whose) .asLongAs
}
