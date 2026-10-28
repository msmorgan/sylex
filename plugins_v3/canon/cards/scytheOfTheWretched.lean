/-- Scythe of the Wretched. -/
card := {
  name := "Scythe of the Wretched",
  cost := [2],
  types := [artifact],
  subtypes := [equipment],
  text := [
    getsPt (equipped creature) (up 2) (up 2),
    whenever
      (does
         (a
            [
              isCreature,
              did (actor (attachHost equipped creature)) (dealDamage anyAmount which)
                (within := some (within soFarThisTurn none))
            ])
         die)
      [returnToBattlefield (that card) [(under you)], attachTo thisEquipment (that creature)],
    equip [4]
  ]
}
