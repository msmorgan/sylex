/-- "attach <what> to it": ordinary pronoun `it` resolved by `D′` (no `Window` override). -/
template attachToIt (what : NounPhrase) : Action := {
  spelling := "attach <what> to it",
  body :=
    attachTo what it
}
