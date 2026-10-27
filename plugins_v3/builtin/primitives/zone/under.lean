/-- "under <controller>'s control" -/
primitive under (controller : NounPhrase) : MoveRider := {
  spelling := "under <controller>'s control",
  body :=
    Semantics.Primitives.MoveRider.under controller
}
