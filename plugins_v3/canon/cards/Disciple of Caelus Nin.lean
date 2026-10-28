/-- Disciple of Caelus Nin: "starting with you" overrides the default APNAP choice order
[CR#101.1,101.4]. -/
card := {
  name := "Disciple of Caelus Nin",
  cost := [4, white],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    when (does thisCreature enter)
      [
        instruct eachPlayer
          (chooseStartingWith you (counted (upTo 5) [isPermanent, controlledBy they])),
        instruct (allOf [isPermanent, otherThan thisCreature, notChosen]) phaseOut
      ],
    cant (bare isPermanent) phaseIn
  ],
  power := 3,
  toughness := 4
}
