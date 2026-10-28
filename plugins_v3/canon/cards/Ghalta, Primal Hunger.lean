card := {
  name := "Ghalta, Primal Hunger",
  cost := [10, green, green],
  supertypes := [legendary],
  types := [creature],
  subtypes := [elder, dinosaur],
  text := [
    static
      [
        costShift this_ (less (letter x)),
        letterDefinition x (aggregate sum (stat power) [isCreature, youControl])
      ],
    trample
  ],
  power := 12,
  toughness := 12
}
