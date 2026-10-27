/-- [CR#725.2] "Whenever a creature deals combat damage to the monarch, its controller becomes
the monarch." -/
template monarchSteal : Conferral := {
  -- TODO(spelling): Whenever a creature deals combat damage to the monarch, its controller becomes
  -- the monarch.; this is a rules conferral, whose inherent or ability-free origin cannot be
  -- encoded by card-text substitution.
  body :=
    .ability
      (whenever (does (a isCreature) (dealDamage (to := theMonarch) (kind := some .combat)))
        (gainDesignation (controllerOf it) monarch))
}
