/-- [CR#704.5v] "If a Siege battle has defense 0 and it isn't the source of an ability that has
triggered but not yet left the stack, it's put into its owner's graveyard." Its defense on the
battlefield is its defense-counter count [CR#310.4c]. -/
template siegeDefenseZero : Conferral := {
  -- TODO(spelling): If a Siege battle has defense 0 and it isn't the source of an ability that has
  -- triggered but not yet left the stack, it's put into its owner's graveyard.; this is a rules
  -- conferral, whose inherent or ability-free origin cannot be encoded by card-text substitution.
  body :=
    .stateBased
      (.and [ .compareAmt (countersOn defenseCounter .this) .eq 0, siegeHasNoWaitingTrigger ])
      (putInto .this graveyard)
}
