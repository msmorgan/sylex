card wardOfPiety : Spelled :=
  { name := "Ward of Piety", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        activated [generic 1, pip .white]
          (establishThroughout
             (replace (would any (dealDamage anyAmount (enchanted creature)))
               (instead (redirectDamage (theNext 1) (target isAnyTarget))))
             thisTurn) ] }
