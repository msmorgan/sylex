/-- "except it isn't legendary" -/
template isntLegendary : CopyExcept := {
  spelling := "except it isn't legendary",
  body :=
    characteristicEdits
      [typeLine loses
          { supertypes := some [legendary] }]
}
