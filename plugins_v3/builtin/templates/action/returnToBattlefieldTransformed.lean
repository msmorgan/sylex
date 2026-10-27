/-- "return <subject> to the battlefield transformed under <controller>'s control" -/
template returnToBattlefieldTransformed (subject controller : NounPhrase) : Action := {
  spelling := "return <subject> to the battlefield transformed under <controller>'s control",
  body :=
    Semantics.Templates.Action.returnToBattlefield subject
      [Semantics.Templates.MoveRider.moveRiderEntersTransformed, Semantics.Templates.MoveRider.under controller]
}
