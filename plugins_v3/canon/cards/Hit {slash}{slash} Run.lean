/-- Hit // Run. Run's affected creatures are fixed as its effect begins ([CR#611.2c]); its "for
each other attacking creature" contrasts with each of them, the current member of the group its
subject names, not with the group, so a lone attacker gets +0/+0. -/
card := split
  {
    left := {
      name := "Hit",
      cost := [1, black, red],
      types := [instant],
      text := [
        spell
          [
            instruct (target isPlayer) (sacrifice (aTheirChoice (or [isArtifact, isCreature]))),
            instruct this_ (dealDamage (statOf (stat manaValue) (that permanent)) (that player))
          ]
      ]
    },
    right := {
      name := "Run",
      cost := [3, red, green],
      types := [instant],
      text := [
        spell
          (get (bare [isAttacking, isCreature, youControl])
             (up (forEach 1 [isAttacking, isCreature, isOther])) (up 0) untilEndOfTurn)
      ]
    }
  }
