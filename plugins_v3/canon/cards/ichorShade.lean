/-- Ichor Shade -/
card ichorShade : Spelled :=
  { name := "Ichor Shade", cost := [generic 2, pip .black], types := [.creature],
    subtypes := [phyrexian, shade],
    text :=
      [ at_ (beginningOfYour .endStep)
          (if_ := some
            (was
              (putInto (a (or [isArtifact, isCreature])) graveyard (from_ := battlefield))
              soFarThisTurn))
          (putCounters 1 p1p1Counter thisCreature) ],
    power := stat 2, toughness := stat 3 }
