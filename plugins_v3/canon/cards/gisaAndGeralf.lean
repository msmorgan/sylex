card := {
  name := "Gisa and Geralf",
  cost := [2, blue, black],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    when (does thisCreature enter) (mill 4),
    can you (cast (counted (exactly 1) [isSpell, isCreature, zombie]) (from_ := yourGraveyard))
      (timing := onEachOfYourTurns)
  ],
  power := 4,
  toughness := 4
}
