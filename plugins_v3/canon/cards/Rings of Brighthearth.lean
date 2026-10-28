card := {
  name := "Rings of Brighthearth",
  cost := [3],
  types := [artifact],
  text := [
    whenever (does you (activate (a (abilityHead anyActivated))))
      (if_ := some (not (is (that ability) isManaAbility)))
      (may (pay [2] once)
         [copyOnStack (that ability), may (chooseNewTargets (that (copied ability)))])
  ]
}
