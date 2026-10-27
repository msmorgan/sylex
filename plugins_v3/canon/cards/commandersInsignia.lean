/-- Commander's Insignia -/
card commandersInsignia : Spelled :=
  { name := "Commander's Insignia", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ static (getsPt (allOf (and [isCreature, youControl]))
          (up (eventCount (cast (headed .spell yourCommander) (from_ :=
            commandZone)) you thisGame))
          (up (eventCount (cast (headed .spell yourCommander) (from_ :=
            commandZone)) you thisGame)))
            ] }
