card := {
  name := "Ivory Tower",
  cost := [1],
  types := [artifact],
  text := [
    at_ (beginningOfYour upkeep)
      [gainLife (letter x), define x (minus (countOf (cardIn yourHand)) 4)]
  ]
}
