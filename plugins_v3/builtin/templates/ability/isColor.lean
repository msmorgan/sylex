/-- "except it's <color>" -/
template isColor (color : Color) : CopyExcept := {
  spelling := "except it's <color>",
  body :=
    edits
      [colors sets (.some [color])]
}
