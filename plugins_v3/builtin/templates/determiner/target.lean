/-- "target …" -/
template target (p : Predicate) : NounPhrase := {
  spelling := "target <p>",
  body :=
    described (target (range (some 1) (some 1))) p
}
