/-- "except it's <types> in addition to its other types" -/
template isAlso (types : List CardType) (subtypes : List Subtype) : CopyExcept := {
  -- TODO(spelling): Format and coordinate the types, subtypes list entries, including the empty
  -- case.
  body :=
    edits
      [typeLine adds
          (Semantics.Characteristics.typeChanges { types := types, subtypes := subtypes })]
}
