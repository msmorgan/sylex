/-- Jem Lightfoote, Sky Explorer -/
card := {
  name := "Jem Lightfoote, Sky Explorer",
  cost := [2, white, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, scout],
  text := [
    flying,
    vigilance,
    at_ (beginningOfYour endStep)
      (if_ := some (not (did you (cast (a isSpell) (from_ := yourHand)) soFarThisTurn))) (draw 1)
  ],
  power := 3,
  toughness := 3
}
