/-- Aurelia, the Law Above -/
card := {
  name := "Aurelia, the Law Above",
  cost := [3, red, white],
  supertypes := [legendary],
  types := [creature],
  subtypes := [angel],
  text := [
    flying,
    vigilance,
    haste,
    whenever (does (a isPlayer) (attackWith (counted (orMore 3) isCreature))) (draw 1),
    whenever (does (a isPlayer) (attackWith (counted (orMore 5) isCreature)))
      [instruct this_ (dealDamage 3 eachOpponent), gainLife 3]
  ],
  power := 4,
  toughness := 4
}
