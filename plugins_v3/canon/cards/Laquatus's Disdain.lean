card := {
  name := "Laquatus's Disdain",
  cost := [1, blue],
  types := [instant],
  text := [spell [counter (target [isSpell, did any (cast which (from_ := graveyard)) thisObject]), draw 1]]
}
