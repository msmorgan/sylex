/-- "a … at random" -/
template aAtRandom (p : Predicate) : NounPhrase := {
  spelling := "a <p> at random",
  body :=
    described (a .atRandom) p
}
