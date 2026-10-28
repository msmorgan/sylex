/-- Tribal Flames -/
card := {
  name := "Tribal Flames",
  cost := [1, red],
  types := [sorcery],
  text := [
    domain
      (spell
         [
           instruct this_ (dealDamage (letter x) (target isAnyTarget)),
           define x (distinctCount (subtype land basicOnly) (allOf [isLand, youControl]))
         ])
  ]
}
