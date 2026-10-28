/-- The monarch: only one player can be the monarch at a time [CR#725.3], so "the monarch" names
one player.

 [CR#725.1,725.2] The monarch: an effect makes a player the monarch, and the monarch draws a
card at the beginning of their end step and loses the designation to the controller of a
creature that deals them combat damage. -/
designation monarch := {
  lexical := { nouns := ["monarch"] },
  spelling := "monarch",
  scope := (.heldBy player),
  sole := true,
  confers :=
    [ability (at_ (beginningOfPossessed endStep theMonarch) (instruct (that player) (draw 1))),
      ability
        (whenever (does (a isCreature) (dealDamage (to := theMonarch) (kind := some combat)))
          (gainDesignation (controllerOf it) monarch))]
}
