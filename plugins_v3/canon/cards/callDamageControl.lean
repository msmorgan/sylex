/-- *Call Damage Control*: the header "Return those cards from your graveyard to your hand." is
the supplied body, written once; each bullet is the target it completes into a mode. -/
card callDamageControl : Spelled :=
  { name := "Call Damage Control", cost := [generic 1, pip .green], types := [.sorcery],
    text :=
      [ spell
          (chooseModesOver (upTo 2)
            [ target (and [isArtifact, isCard]),
              target (and [isCreature, isCard]),
              target (and [isEnchantment, isCard]),
              target (and [isLand, isCard]) ]
            (fun cards => returnToHand cards yourGraveyard)) ] }
