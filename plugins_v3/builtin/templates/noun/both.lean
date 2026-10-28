/-- "both <left> and <right>" -/
template both (left right : NounPhrase) : NounPhrase := {
  spelling := "both <left> and <right>",
  body :=
    and [left, right]
}
