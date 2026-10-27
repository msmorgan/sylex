/-- Hit // Run. Run's affected creatures are fixed as its effect begins ([CR#611.2c]); its "for
each other attacking creature" contrasts with each of them, the current member of the group its
subject names, not with the group, so a lone attacker gets +0/+0. -/
card hitRun : Spelled := { body := .split
  { characteristics :=
    { name := "Hit", cost := [generic 1, pip .black, pip .red], types := [.instant],
      text :=
        [ spell (sequentially
            [ instruct (target isPlayer) (sacrifice (aTheirChoice (or [isArtifact, isCreature]))),
              instruct this_
                (dealDamage (statOf (.stat .manaValue) (that permanent)) (that player)) ]) ] } }
  { characteristics :=
    { name := "Run", cost := [generic 3, pip .red, pip .green], types := [.instant],
      text :=
        [ spell (get (bare (and [isAttacking, isCreature, youControl]))
            (up (forEach 1 (and [isAttacking, isCreature, isOther]))) (up 0) untilEndOfTurn) ] } } }
