/-- The results of damage dealt to `this`, `result` reading the damage dealt as `thatMuch`. -/
template damageResult (result : Action) : Conferral := {
  -- TODO(spelling): Damage dealt to this permanent causes <result>; this conferral describes a rule
  -- of the game, whose ability-free origin cannot be encoded by card-text substitution.
  body :=
    .property (asEvent (does any (dealDamage anyAmount this_)) result)
}
