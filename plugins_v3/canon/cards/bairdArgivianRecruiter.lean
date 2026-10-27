/-- Baird, Argivian Recruiter: "if you control a creature with power greater than its base
power", each creature measured against its own base power [CR#208.4b]. -/
card bairdArgivianRecruiter : Spelled :=
  { name := "Baird, Argivian Recruiter", cost := [pip .red, pip .white],
    supertypes := [.legendary], types := [.creature], subtypes := [human, soldier],
    text :=
      [ at_ (beginningOfYour .endStep)
          (if_ := exists_ (comparesOwnStat .power (and [isCreature, youControl])
            .greater (basePowerOf ownMember)))
          (create 1 (creatureToken 1 1 [.white] [soldier])) ],
    power := stat 2, toughness := stat 2 }
