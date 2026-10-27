/-- Watcher in the Web -/
card watcherInTheWeb : Spelled :=
  { name := "Watcher in the Web", cost := [generic 4, pip .green], types := [.creature],
    subtypes := [spider],
    text := [reach, static
               (can thisCreature (block (counted (additional (exactly 7)) isCreature))
                  (timing := eachCombat))],
    power := stat 2, toughness := stat 5 }
