/-- "except it isn't legendary" -/
template isntLegendary : CopyExcept := {
  spelling := "except it isn't legendary",
  body :=
    Semantics.Templates.CopyExcept.copyExceptEdits
      [Semantics.Templates.CharacteristicEdit.characteristicEditTypeLine Semantics.QualityOp.loses
          { supertypes := Option.some [Semantics.Supertype.legendary] }]
}
