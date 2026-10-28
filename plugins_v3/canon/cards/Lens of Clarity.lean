card := {
  name := "Lens of Clarity",
  cost := [1],
  types := [artifact],
  text := [
    static
      [
        visibility lookAt you topOfLibrary,
        visibility lookAt you (objects (allOf [isCreature, isFaceDown, youDontControl]))
      ]
  ]
}
