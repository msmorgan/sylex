/-- Stone Haven Outfitter -/
card stoneHavenOutfitter : Spelled :=
  { name := "Stone Haven Outfitter", cost := [generic 1, pip .white], types := [.creature],
    subtypes := [kor, artificer, ally],
    text :=
      [ static (getsPt (allOf (and [and [isCreature, youControl], isAttached (some .equipped)]))
          (up 1) (up 1)),
        whenever (does (a (and [and [isCreature, youControl], isAttached (some .equipped)])) die)
          (draw 1) ],
    power := stat 2, toughness := stat 2 }
