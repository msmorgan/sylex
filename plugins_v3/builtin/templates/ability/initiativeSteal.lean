/-- [CR#726.2] "Whenever one or more creatures a player controls deal combat damage to the player
who has the initiative, the controller of those creatures takes the initiative." -/
template initiativeSteal : Conferral := {
  -- TODO(spelling): Whenever one or more creatures a player controls deal combat damage to the
  -- player who has the initiative, the controller of those creatures takes the initiative.; this is
  -- a rules conferral, whose inherent or ability-free origin cannot be encoded by card-text
  -- substitution.
  body :=
    .ability
      (whenever
        (does (counted (orMore 1) (and [isCreature, controlledBy (a isPlayer)]))
          (dealDamage (to := theInitiativeHolder) (kind := some .combat)))
        (gainDesignation (controllerOf (those creature)) initiative))
}
