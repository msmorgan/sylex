/-- Commander's Insignia -/
card := {
  name := "Commander's Insignia",
  cost := [2, white, white],
  types := [enchantment],
  text := [
    getsPt (allOf [isCreature, youControl])
      (up (eventCount (cast (headed spell yourCommander) (from_ := commandZone)) you thisGame))
      (up (eventCount (cast (headed spell yourCommander) (from_ := commandZone)) you thisGame))
  ]
}
