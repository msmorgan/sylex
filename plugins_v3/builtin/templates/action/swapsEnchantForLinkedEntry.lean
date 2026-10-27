template swapsEnchantForLinkedEntry : Action := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := establish (conjunction
    [ abilityLoss it [written (enchant creatureCardInAGraveyard)],
      abilityGrant it (enchant (creaturePutOntoTheBattlefieldWith thisAura)) ]
    (subject := it))
}
