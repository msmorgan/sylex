/-- "except it's <types> in addition to its other types" -/
template isAlso (types : List CardType) (subtypes : List Subtype) : CopyExcept := {
  -- TODO(spelling): except it's <types> in addition to its other types; wording varies with subtypes, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    copyExceptEdits
      [characteristicEditTypeLine Semantics.QualityOp.adds
          (Semantics.Characteristics.typeChanges { types := types, subtypes := subtypes })]
}
