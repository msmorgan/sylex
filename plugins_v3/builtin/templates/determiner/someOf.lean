/-- "N of <group>" -/
template someOf (quantity : Quantity) (group : NounPhrase) : NounPhrase := {
  -- TODO(spelling): N of <group>; wording varies with quantity, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Templates.NounPhrase.nounPhraseSomeOf (Semantics.Templates.SliceCount.sliceCountCounted quantity) Option.none group
}
