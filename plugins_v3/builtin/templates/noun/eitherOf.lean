/-- "<left> or <right>" -/
template eitherOf (left right : NounPhrase) : NounPhrase := {
  spelling := "<left> or <right>",
  body :=
    or [left, right]
}
