/-- Mistbreath Elder -/
card mistbreathElder : Spelled :=
  { name := "Mistbreath Elder", cost := [pip .green], types := [.creature],
    subtypes := [frog, warrior],
    text :=
      [ at_ (beginningOfYour .upkeep)
          (doIfDone (returnToHand (a (and [isCreature, youControl, isOther])))
            (putCounters 1 p1p1Counter thisCreature)
            (may (returnToHand thisCreature))) ],
    power := stat 2, toughness := stat 2 }
