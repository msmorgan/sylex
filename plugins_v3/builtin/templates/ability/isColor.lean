/-- "except it's <color>" -/
template isColor (color : Color) : CopyExcept := {
  spelling := "except it's <color>",
  body :=
    copyExceptEdits
      [characteristicEditColors Semantics.QualityOp.sets (Semantics.ColorSpec.some [color])]
}
