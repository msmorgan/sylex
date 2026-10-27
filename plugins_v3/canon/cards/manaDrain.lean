/-- Mana Drain: "Counter target spell. At the beginning of your next main phase, add an
amount of {C} equal to that spell's mana value." The delayed read names the departed spell
[CR#608.2h]. -/
card manaDrain : Spelled :=
  { name := "Mana Drain", cost := [pip .blue, pip .blue], types := [.instant],
    text := [spell (sequentially
      [ counter (target isSpell),
        delay (beginningOf .mainPhase (some you) (nextTime (some you)))
          (addMana (statOf (.stat .manaValue) (that .spell)) (manaOf [[.colorless]])) ])] }
