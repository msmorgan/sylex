template inPile (pile : NounPhrase) : Predicate := {
  spelling := "in <pile>",
  body :=
    in_ (thePile pile)
}
