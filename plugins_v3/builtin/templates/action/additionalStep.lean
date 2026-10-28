/-- "there is an additional <part> [after <anchor>]": steps added to the current turn's phase
[CR#500.9]. -/
-- [CR#500.9]
template additionalStep (part : TurnPart) (anchor : Option TurnPart) (count : Amount) :
    Action := {
  -- TODO(spelling): Context-dependent wording: there is an additional <part> [after <anchor>]; string substitution cannot express its omitted or inflected parts.
  body := addTurnPart part count anchor none
}
