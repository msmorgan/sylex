/-- "As <subject> enters, choose a <quality>." -/
template entersChoosing (subject : NounPhrase) (sort : QualitySort) : StaticSpec := {
  -- TODO(spelling): Context-dependent wording: As <subject> enters, choose a <quality>.; string substitution cannot express its omitted or inflected parts.
  body :=
    entryChoice subject (a (quality sort))
}
