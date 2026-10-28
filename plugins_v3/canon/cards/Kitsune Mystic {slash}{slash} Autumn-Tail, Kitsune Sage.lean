card := flip
  {
    normal := {
      name := "Kitsune Mystic",
      cost := [3, white],
      types := [creature],
      subtypes := [fox, wizard],
      text := [
        (at_ (beginningOfEach endStep)
           (if_ := is thisCreature (attachedBy (some enchanted) (counted (orMore 2) aura)))
           (flip thisCreature))
      ],
      power := 2,
      toughness := 3
    },
    alternative := {
      name := "Autumn-Tail, Kitsune Sage",
      supertypes := [legendary],
      types := [creature],
      subtypes := [fox, wizard],
      text := [
        activated [1]
          (attachTo (target [aura, attachedTo (a isCreature)]) (a [isCreature, isOther]))
      ],
      power := 4,
      toughness := 5
    }
  }
