/-- Conspicuous Snoop -/
card conspicuousSnoop : Spelled :=
  { name := "Conspicuous Snoop", cost := [pip .red, pip .red], types := [.creature],
    subtypes := [goblin, rogue],
    text :=
      [ static (visibility .reveal you topOfLibrary),
        static (can you
                  (play (allOf (and [isSpell, goblin])) (from_ := onTop))),
        static (onlyWhile
          (abilityGrantFrom thisCreature (topSlice 1) [.anyActivated])
          (is (topSlice 1) goblin)) ],
    power := stat 2, toughness := stat 2 }
