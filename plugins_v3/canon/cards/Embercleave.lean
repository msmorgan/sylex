card := {
  name := "Embercleave",
  cost := [4, red, red],
  supertypes := [legendary],
  types := [artifact],
  subtypes := [equipment],
  text := [
    flash,
    costShift this_ (less (forEach 1 [isCreature, isAttacking, youControl])),
    when (does thisEquipment enter) (attachTo it (target [isCreature, youControl])),
    conjunction
      [
        getsPt (equipped creature) (up 1) (up 1),
        abilityGrant (equipped creature) doubleStrike,
        abilityGrant (equipped creature) trample
      ]
      (subject := equipped creature),
    equip [3]
  ]
}
