card grievousWoundLifeLock : Spelled :=
  { name := "Grievous Wound", cost := [generic 3, pip .black, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isPlayer,
        static (cant (enchanted player) (gainLife anyAmount)),
        whenever (does any (dealDamage (to := enchanted player)))
          (instruct they (loseLife (half .up (lifeTotalOf they)))) ] }
