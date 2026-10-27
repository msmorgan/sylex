/-- "enters <status>", as "enters tapped" -/
primitive entersAs (status : Status) : MoveRider := {
  spelling := "enters <status>",
  body :=
    Semantics.Primitives.MoveRider.entersAs status
}
