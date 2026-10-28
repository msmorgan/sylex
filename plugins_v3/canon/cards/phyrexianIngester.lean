/-- Phyrexian Ingester -/
card := {
  name := "Phyrexian Ingester",
  cost := [6, blue],
  types := [creature],
  subtypes := [phyrexian, beast],
  text := [
    imprint (when (does thisCreature enter) (may (exile (target [isCreature, isNontoken])))),
    static
      [
        getsPt thisCreature (up (letter x)) (up (letter y)),
        letterDefinition x (powerOf (the [isCreature, in_ (exiledWith thisCreature)])),
        letterDefinition y (toughnessOf (that card))
      ]
  ],
  power := 3,
  toughness := 3
}
