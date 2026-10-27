/-- "<player>'s life total becomes <amount>" -/
template setLife (amount : Amount) : Action := {
  -- TODO(spelling): Context-dependent wording: <player>'s life total becomes <amount>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.changeLife (Semantics.Templates.setTo amount)
}
