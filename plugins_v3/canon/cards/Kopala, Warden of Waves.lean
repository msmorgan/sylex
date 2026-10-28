card := {
  name := "Kopala, Warden of Waves",
  cost := [1, blue, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [merfolk, wizard],
  text := [
    costShift
      (allOf
         [
           isSpell,
           did yourOpponents (cast which) thisObject,
           targeting (a [isCreature, merfolk, youControl]) someTarget
         ])
      (more 2),
    costShift
      (allOf
         [
           abilityHead anyActivated,
           did yourOpponents (activate which) thisObject,
           targeting (a [isCreature, merfolk, youControl]) someTarget
         ])
      (more 2)
  ],
  power := 2,
  toughness := 2
}
