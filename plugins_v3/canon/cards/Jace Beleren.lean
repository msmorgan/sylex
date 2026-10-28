/-- Pins the planeswalker entry conferral [CR#306.5b] and loyalty costs
[CR#606.4], with loyalty-ability timing and the once-per-turn restriction [CR#606.3].
Zero loyalty puts the planeswalker into its owner’s graveyard [CR#704.5i]. -/
card := {
  name := "Jace Beleren",
  cost := [1, blue, blue],
  supertypes := [legendary],
  types := [planeswalker],
  subtypes := [jace],
  text := [
    activated (loyaltySymbol (up 2)) (instruct eachPlayer (draw 1)),
    activated (loyaltySymbol (down 1)) (instruct (target isPlayer) (draw 1)),
    activated (loyaltySymbol (down 10)) (instruct (target isPlayer) (mill 20))
  ],
  loyalty := 3
}
