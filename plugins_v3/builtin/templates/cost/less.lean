/-- "costs <amount> less[. This effect can't reduce the mana in that cost to less than <floor>
mana]" -/
template less (amount : Amount) (floor : Option Amount := none) : CostShift := {
  -- TODO(spelling): Include or omit the clauses supplied by floor.
  body :=
    generic (down amount) floor
}
