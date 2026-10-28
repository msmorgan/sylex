/-- "You may play this card from <zone> <timing>": the land type's play permission [CR#305.1]. -/
-- [CR#305.1]
template playableFrom (zone : Location) (timing : Timing) : Conferral := {
  spelling := "You may play this card from <zone> <timing>",
  body :=
    Semantics.Conferral.property
      (can (actor you)
        (procedure Semantics.Procedure.play this_ itsOwnCost
          none (some zone) none)
        (some timing))
}
