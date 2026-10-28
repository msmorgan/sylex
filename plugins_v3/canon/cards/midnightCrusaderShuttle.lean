/-- Midnight Crusader Shuttle: "If you gain control of a creature this way" reads the grant in
the second arm of the villainous choice [CR#608.2c,613.1b]. "Midnight Entity" is a flavor word
[CR#207.2d]. -/
card := {
  name := "Midnight Crusader Shuttle",
  cost := [4],
  types := [artifact],
  subtypes := [vehicle],
  text := [
    flavorWord "Midnight Entity"
      (whenever (does thisVehicle attack)
         [
           faceAVillainousChoice (instruct defendingPlayer (sacrifice (a isCreature)))
             (gainControl (target [isCreature, controlledBy defendingPlayer]) untilEndOfTurn),
           doIf (did you (gainControlOf (a isCreature)) thisWay)
             [tap it, becomeAttacking it (some defendingPlayer)]
         ]),
    crew 2
  ],
  power := 3,
  toughness := 4
}
