/-- Promise of Bunrei -/
card promiseOfBunrei : Spelled :=
  { name := "Promise of Bunrei", cost := [generic 2, pip .white], types := [.enchantment],
    text :=
      [ when (does (a (and [isCreature, youControl])) die)
          (doIfDone (sacrifice thisEnchantment)
            (create 4 (creatureToken 1 1 [] [spirit]))) ] }
