/-- "As <subject> enters, choose a <quality> from <domain>." -/
template entersChoosingFrom (subject : NounPhrase) (sort : QualitySort)
    (domain : ChoiceDomain) : StaticSpec := {
  -- TODO(spelling): Context-dependent wording: As <subject> enters, choose a <quality> from <domain>.; string substitution cannot express its omitted or inflected parts.
  body :=
    entryChoice subject (a (qualityFrom sort domain))
}
