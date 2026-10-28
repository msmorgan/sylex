/-- "<subject> dies": put into a graveyard from the battlefield [CR#700.4]. -/
-- [CR#700.4]
template die : Action := {
  spelling := "dies",
  body :=
    move theActor (zone battlefield none)
      (some (zone graveyard none)) .departure []
}
