/-- "except it isn't legendary" -/
template isntLegendary : CopyExcept := {
  spelling := "except it isn't legendary",
  body :=
    edits
      [typeLine loses
          { supertypes := some [legendary] }]
}
