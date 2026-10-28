/-- "<subject> enters tapped unless <condition>" -/
template entersTappedUnless (subject : NounPhrase) (condition : Condition) : StaticSpec := {
  spelling := "<subject> enters tapped unless <condition>",
  body :=
    entersUnless subject [entersAs Semantics.Status.tapped] condition
}
