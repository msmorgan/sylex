/-- "<subject> enters tapped unless <condition>" -/
template entersTappedUnless (subject : NounPhrase) (condition : Condition) : StaticSpec := {
  spelling := "<subject> enters tapped unless <condition>",
  body :=
    Semantics.Templates.StaticSpec.entersUnless subject [Semantics.Templates.MoveRider.entersAs Semantics.Status.tapped] condition
}
