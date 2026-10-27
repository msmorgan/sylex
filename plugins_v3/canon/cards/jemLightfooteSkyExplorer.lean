/-- Jem Lightfoote, Sky Explorer -/
card jemLightfooteSkyExplorer : Spelled :=
  { name := "Jem Lightfoote, Sky Explorer", cost := [generic 2, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, scout],
    text :=
      [ flying,
        vigilance,
        at_ (beginningOfYour .endStep)
          (if_ := some (not
             (did you (cast (a isSpell) (from_ := yourHand)) soFarThisTurn)))
          (draw 1) ],
    power := stat 3, toughness := stat 3 }
