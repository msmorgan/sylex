/-- "<subject> enters tapped" -/
template entersTapped (subject : NounPhrase) : StaticSpec := {
  spelling := "<subject> enters tapped",
  body :=
    enters subject [entersAs Semantics.Status.tapped]
}
