/-- "costs <amount> less[. This effect can't reduce the mana in that cost to less than <floor>
mana]" -/
template less (amount : Amount) (floor : Option Amount := none) : CostShift := {
  -- TODO(spelling): Context-dependent wording: costs <amount> less[. This effect can't reduce the mana in that cost to less than <floor> mana]; string substitution cannot express its omitted or inflected parts.
  body :=
    generic (down amount) floor
}
