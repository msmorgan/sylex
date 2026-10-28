/-- Arcane Proxy -/
card := prototype
  {
    inner := {
      name := "Arcane Proxy",
      cost := [7],
      types := [artifact, creature],
      subtypes := [wizard],
      text := [
        prototype,
        when (does thisCreature enter) (if_ := is it (did you (cast which) thisObject))
          [
            exile
              (target
                 [
                   isInstantOrSorcery,
                   isCard,
                   hasManaValue atMost (powerOf thisCreature),
                   in_ yourGraveyard
                 ]),
            copy fromCardZone (that card) 1 [],
            may (cast (that copy) (payment := withoutPaying))
          ]
      ],
      power := 4,
      toughness := 3
    },
    alternative := { cost := [1, blue, blue], power := 2, toughness := 1 }
  }
