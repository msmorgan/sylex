/-- "<subject> leaves <from_>", the battlefield unless said. -/
template leave (from_ : Location := battlefield) : Action := {
  spelling := "leaves <from_>",
  body :=
    move theActor from_ none .departure []
}
