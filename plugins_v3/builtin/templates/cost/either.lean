/-- "<left> or <right>" as a cost -/
template either (left right : Cost) : Cost := {
  spelling := "<left> or <right>",
  body :=
    or [left, right]
}
