/-- Phyrexian Rebirth -/
card := {
  name := "Phyrexian Rebirth",
  cost := [4, white, white],
  types := [sorcery],
  text := [
    spell
      [
        destroy (allOf isCreature),
        create 1
          (written
             {
               types := [artifact, creature],
               subtypes := [phyrexian, horror],
               power := letter x,
               toughness := (letter x)
             }),
        define x (countOf [isCreature, was (destroy which) thisWay])
      ]
  ]
}
