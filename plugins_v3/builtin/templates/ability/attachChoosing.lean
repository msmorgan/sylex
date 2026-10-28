/-- "As <subject> becomes attached, choose a <quality>." -/
template attachChoosing (subject : NounPhrase) (sort : QualitySort) : StaticSpec := {
  -- TODO(spelling): Context-dependent wording: As <subject> becomes attached, choose a <quality>.; string substitution cannot express its omitted or inflected parts.
  body :=
    attachmentChoice subject (a (quality sort))
}
