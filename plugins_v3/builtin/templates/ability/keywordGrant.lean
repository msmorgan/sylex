/-- "<subject> has <term>", for a term a "the same is true for" list names. A keyword named bare is
granted itself; a family term ("landwalk", "protection", "hexproof") grants each variant of it
that the objects `holders` describes have, "the <term> abilities of those objects" [CR#702.1c]. -/
-- [CR#702.1c]
template keywordGrant (subject : NounPhrase) (term : KeywordTerm)
    (holders : Predicate) : StaticSpec := {
  -- TODO(spelling): <subject> has <term>; wording varies with holders, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    match term with
    | Semantics.KeywordTerm.the label => Semantics.Templates.StaticSpec.abilityGrant subject (Semantics.Templates.keyword label)
    | x =>
      Semantics.Templates.StaticSpec.abilityGrantSelected subject (Semantics.Templates.NounPhrase.allOf holders)
        (Semantics.Templates.AbilitySelection.abilitySelectionSpecified [Semantics.Templates.AbilityLost.abilities term])
}
