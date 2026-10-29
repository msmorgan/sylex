/-- "except it's <power>/<toughness>" -/
template hasPT (power toughness : Amount) : CopyExcept := {
  spelling := "except it's <power>/<toughness>",
  body :=
    edits
      [stat power (set power),
        stat toughness (set toughness)]
}
