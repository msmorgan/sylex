card := {
  name := "Balemurk Leech",
  cost := [1, black],
  types := [creature],
  subtypes := [leech],
  text := [
    eerie
      (whenever (does (a [isEnchantment, youControl]) enter)
         (joins := [joinedHead (does you (fullyUnlock (a room)))])
         (instruct eachOpponent (loseLife 1)))
  ],
  power := 2,
  toughness := 2
}
