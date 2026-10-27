/-- "except it's <color>" -/
template isColor (color : Color) : CopyExcept := {
  spelling := "except it's <color>",
  body :=
    Semantics.Templates.CopyExcept.copyExceptEdits
      [Semantics.Templates.CharacteristicEdit.characteristicEditColors Semantics.QualityOp.sets (Semantics.ColorSpec.some [color])]
}
