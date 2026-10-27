/-- Disciple of Caelus Nin: "starting with you" overrides the default APNAP choice order
[CR#101.1,101.4]. -/
card discipleOfCaelusNin : Spelled :=
  { name := "Disciple of Caelus Nin", cost := [generic 4, pip .white], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ when (does thisCreature enter)
          (sequentially
            [ instruct eachPlayer
                (chooseStartingWith you
                   (counted (upTo 5) (and [isPermanent, controlledBy they]))),
              instruct (allOf (and [isPermanent, otherThan thisCreature, notChosen]))
                phaseOut ]),
        static (cant (bare isPermanent) phaseIn) ],
    power := stat 3, toughness := stat 4 }
