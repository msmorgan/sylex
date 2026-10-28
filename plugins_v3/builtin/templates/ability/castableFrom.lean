/-- "You may cast this card from <zone> <timing>": the instant type's cast permission
[CR#304.1]. -/
-- [CR#304.1]
template castableFrom (zone : Location) (timing : Timing) : Conferral := {
  spelling := "You may cast this card from <zone> <timing>",
  body :=
    Semantics.Conferral.property
      (can (actor you)
        (procedure Semantics.Procedure.cast this_ itsOwnCost
          none (some zone) none)
        (some timing))
}
