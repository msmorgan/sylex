card curseOfTheBloodyTome : Spelled :=
  { name := "Curse of the Bloody Tome", cost := [generic 2, pip .blue],
    types := [.enchantment], subtypes := [aura, curse],
    text :=
      [ enchant isPlayer,
        at_ (beginningOfPossessed .upkeep (enchanted player))
          (instruct (that player) (mill 2)) ] }
