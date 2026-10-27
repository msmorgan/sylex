/-- Zidane, Tantalus Thief. "Gain control of" is the imperative `gainControl`, a Continuous Effect
[CR#611.2a,613.1b]; "an opponent gains control of a permanent from you" names the control change
as a trigger event, `gainControlOf`, with the player it was taken from [CR#603.10d]. -/
card zidaneTantalusThief : Spelled :=
  { name := "Zidane, Tantalus Thief", cost := [generic 3, pip .red, pip .white],
    supertypes := [.legendary], types := [.creature], subtypes := [human, mutant, scout],
    text :=
      [ when (does thisCreature enter)
          (sequentially
            [ gainControl (target (and [isCreature, anOpponentControls])) untilEndOfTurn,
              untap it,
              establishFor it [abilityGrant it lifelink, abilityGrant it haste]
                untilEndOfTurn ]),
        whenever (does anOpponent (gainControlOf (a isPermanent) you))
          (create 1 treasureToken) ],
    power := stat 3, toughness := stat 3 }
