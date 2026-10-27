/-- "As <subject> enters, choose a <quality>." -/
template entersChoosing (subject : NounPhrase) (sort : QualitySort) : StaticSpec := {
  -- TODO(spelling): Context-dependent wording: As <subject> enters, choose a <quality>.; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.StaticSpec.entryChoice subject (Semantics.Templates.NounPhrase.a (Semantics.Templates.Predicate.quality sort))
}
