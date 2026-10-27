card lashknifeBarrier : Spelled :=
  { name := "Lashknife Barrier", cost := [generic 2, pip .white], types := [.enchantment],
    text :=
      [ when (does thisEnchantment enter) (draw 1),
        static
          (replace (would (a isSource) (dealDamage anyAmount (a (and [isCreature, youControl]))))
            (instead (instruct it (dealDamage (minus thatMuch 1) (that creature))))) ] }
