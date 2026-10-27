/-- Blinding Angel {3}{W}{W} — Creature — Angel 2/4. "Flying. Whenever Blinding Angel deals
combat damage to a player, that player skips their next combat phase." -/
card blindingAngel : Spelled :=
  { name := "Blinding Angel", cost := [generic 3, pip .white, pip .white],
    types := [.creature], subtypes := [angel],
    text :=
      [ flying,
        whenever
          (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
          (instruct (that player) (skipPart .combat 1)) ],
    power := stat 2, toughness := stat 4 }
