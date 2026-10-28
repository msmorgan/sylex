/-- "except it isn't legendary" -/
template isntLegendary : CopyExcept := {
  spelling := "except it isn't legendary",
  body :=
    copyExceptEdits
      [characteristicEditTypeLine Semantics.QualityOp.loses
          { supertypes := some [Semantics.Supertype.legendary] }]
}
