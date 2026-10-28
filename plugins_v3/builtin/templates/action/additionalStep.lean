/-- "there is an additional <part> [after <anchor>]": steps added to the current turn's phase
[CR#500.9]. -/
-- [CR#500.9]
template additionalStep (part : TurnPart) (anchor : Option TurnPart) (count : Amount) :
    Action := {
  -- TODO(spelling): Include or omit the clauses supplied by anchor.
  body := addTurnPart part count anchor none
}
