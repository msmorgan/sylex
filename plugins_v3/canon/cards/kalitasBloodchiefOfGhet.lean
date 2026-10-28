/-- Kalitas, Bloodchief of Ghet: "{B}{B}{B}, {T}: Destroy target creature. If that creature dies
this way, create a black Vampire creature token. Its power is equal to that creature's power and
its toughness is equal to that creature's toughness." "That creature" in the lookback is the
destroyed creature, read as the permanent it was. -/
card := {
  name := "Kalitas, Bloodchief of Ghet",
  cost := [5, black, black],
  supertypes := [legendary],
  types := [creature],
  subtypes := [vampire, warrior],
  text := [
    lifelink,
    activated (compound [[black, black, black], tapSymbol])
      [
        destroy (target isCreature),
        doIf (did (that creature) die thisWay)
          (create 1
             (creatureTokenOf (powerOf (that creature)) (toughnessOf (that creature)) [black]
                [vampire]))
      ]
  ],
  power := 5,
  toughness := 5
}
