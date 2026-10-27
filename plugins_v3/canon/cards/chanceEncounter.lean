card chanceEncounter : Spelled :=
  { name := "Chance Encounter", cost := [generic 2, pip .red, pip .red],
    types := [.enchantment],
    text :=
      [ whenever (does you (flipCoins anyAmount (some .wins)))
          (putCounters 1 luckCounter thisEnchantment),
        at_ (beginningOfYour .upkeep)
          (if_ := compareAmt (countersOn luckCounter thisEnchantment) .atLeast 10)
          winGame ] }
