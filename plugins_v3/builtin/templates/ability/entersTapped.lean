/-- "<subject> enters tapped" -/
template entersTapped (subject : NounPhrase) : StaticSpec := {
  spelling := "<subject> enters tapped",
  body :=
    Semantics.Templates.StaticSpec.enters subject [Semantics.Templates.MoveRider.entersAs Semantics.Status.tapped]
}
