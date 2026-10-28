/-- "take <count> extra turns after this one": the performer takes them [CR#500.7]. -/
-- [CR#500.7]
template extraTurn (count : Amount) : Action := {
  spelling := "take <count> extra turns after this one",
  body := addTurnPart turn count none none
}
