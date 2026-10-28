card := {
  name := "Saw in Half",
  cost := [2, black],
  types := [instant],
  text := [
    spell
      [
        destroy (target isCreature),
        doIf (did (that creature) die thisWay)
          (instruct (controllerOf it)
             (create 2
                (copyOf (that creature)
                   (copyCharacteristics
                      {
                        power := half up (powerOf (that creature)),
                        toughness := half up (toughnessOf (that creature))
                      }
                      false))
                []))
      ]
  ]
}
