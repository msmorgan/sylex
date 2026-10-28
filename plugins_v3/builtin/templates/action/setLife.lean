/-- "<player>'s life total becomes <amount>" -/
template setLife (amount : Amount) : Action := {
  -- TODO(spelling): The Actor supplies the possessor of 'life total'; no subject parameter is
  -- available in this declaration.
  body :=
    changeLife (setTo amount)
}
