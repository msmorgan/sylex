/-- "<player>'s library": a face-down pile [CR#401.2]. -/
-- [CR#401.2]
primitive libraryOf (player : NounPhrase) : Location := {
  spelling := "<player>'s library",
  body :=
    Semantics.Primitives.Location.zone Semantics.Zone.library (Option.some player)
}
