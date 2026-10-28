card := {
  name := "Unstable Shapeshifter",
  cost := [3, blue],
  types := [creature],
  subtypes := [shapeshifter],
  text := [
    whenever (does (a [isCreature, isOther]) enter)
      (establish (copyChange thisCreature (that creature) [exceptThisAbility]))
  ],
  power := 0,
  toughness := 1
}
