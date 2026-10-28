/-- "except it isn't legendary" -/
template isntLegendary : CopyExcept := {
  spelling := "except it isn't legendary",
  body :=
    copyExceptEdits
      [characteristicEditTypeLine .loses
          { supertypes := some [.legendary] }]
}
