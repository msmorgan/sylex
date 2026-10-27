/-- "<pile>": one of the piles a separation made, as a place cards go. -/
primitive thePile (pile : NounPhrase) : Location := {
  spelling := "<pile>",
  body :=
    Semantics.Primitives.Location.pile pile
}
