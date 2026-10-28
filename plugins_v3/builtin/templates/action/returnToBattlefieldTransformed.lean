/-- "return <subject> to the battlefield transformed under <controller>'s control" -/
template returnToBattlefieldTransformed (subject controller : NounPhrase) : Action := {
  spelling := "return <subject> to the battlefield transformed under <controller>'s control",
  body :=
    returnToBattlefield subject
      [moveRiderEntersTransformed, under controller]
}
