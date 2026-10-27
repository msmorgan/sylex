/-- "the <p> [from] among <group>": every member the description picks. -/
template allFromAmong (p : Predicate) (group : NounPhrase) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: the <p> [from] among <group>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.NounPhrase.nounPhraseSomeOf Semantics.Templates.SliceCount.whole (Option.some p) group
}
