card := {
  name := "Destructive Revelry",
  cost := [red, green],
  types := [instant],
  text := [
    spell
      [
        destroy (target (or [isArtifact, isEnchantment])),
        instruct this_ (dealDamage 2 (controllerOf (that permanent)))
      ]
  ]
}
