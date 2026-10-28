/-- The actor separates the group into the specified number of piles. -/
primitive separateIntoPiles (group : NounPhrase) (piles : Nat)
    (faces : List PileFace := []) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.separateIntoPiles group piles faces
}
