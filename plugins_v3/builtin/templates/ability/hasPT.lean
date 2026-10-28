/-- "except it's <power>/<toughness>" -/
template hasPT (power toughness : Amount) : CopyExcept := {
  spelling := "except it's <power>/<toughness>",
  body :=
    edits
      [stat sets power power,
        stat sets toughness toughness]
}
