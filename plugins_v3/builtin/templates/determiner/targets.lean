/-- "<q> target <p>s", as "up to two target creatures" or "any number of targets". -/
template targets (q : Quantity) (p : Predicate) : NounPhrase := {
  spelling := "<q> target <p>",
  body :=
    described (target q) p
}
