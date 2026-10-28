/-- Ragavan, Nimble Pilferer. Dash's "return the permanent this spell becomes" is a Resolution
Delayed Trigger [CR#702.109a,608.3g]. -/
card := {
  name := "Ragavan, Nimble Pilferer",
  cost := [red],
  supertypes := [legendary],
  types := [creature],
  subtypes := [monkey, pirate],
  text := [
    whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some combat)))
      [
        create 1 treasureToken,
        exile (slice top 1 (libraryOf (that player))),
        establishThroughout (can you (cast (that card))) untilEndOfTurn
      ],
    dash [1, red]
  ],
  power := 2,
  toughness := 1
}
