/-- "in <pile>": a Predicate selecting Objects in the named pile. -/
template inPile (pile : NounPhrase) : Predicate := {
  spelling := "in <pile>",
  body :=
    in_ (thePile pile)
}
