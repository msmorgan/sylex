/-- Pins domain counting basic land types [CR#205.3i]. The ability word itself
has no rules meaning [CR#207.2c]. -/
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
