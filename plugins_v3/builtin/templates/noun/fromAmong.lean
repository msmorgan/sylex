/-- "N <p> from among <group>" -/
template fromAmong (quantity : Quantity) (p : Predicate) (group : NounPhrase) :
    NounPhrase := {
  -- TODO(spelling): N <p> from among <group>; wording varies with quantity, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Templates.NounPhrase.nounPhraseSomeOf (Semantics.Templates.SliceCount.sliceCountCounted quantity) (Option.some p) group
}
