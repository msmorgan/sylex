/-- "N of <group>" -/
template someOf (quantity : Quantity) (group : NounPhrase) : NounPhrase := {
  -- TODO(spelling): N of <group>; wording varies with quantity, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    nounPhraseSomeOf (sliceCountCounted quantity) none group
}
