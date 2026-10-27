template inPile (pile : NounPhrase) : Predicate := {
  spelling := "in <pile>",
  body :=
    Semantics.Templates.Predicate.in_ (Semantics.Templates.Location.thePile pile)
}
