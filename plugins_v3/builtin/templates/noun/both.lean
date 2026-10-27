/-- "both <left> and <right>" -/
template both (left right : NounPhrase) : NounPhrase := {
  spelling := "both <left> and <right>",
  body :=
    Semantics.Templates.NounPhrase.and [left, right]
}
