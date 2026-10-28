/-- Up the Beanstalk -/
card := {
  name := "Up the Beanstalk",
  cost := [1, green],
  types := [enchantment],
  text := [
    when (does this_ enter)
      (joins := [joinedHead (does you (cast (a [isSpell, hasManaValue atLeast 5])))]) (draw 1)
  ]
}
