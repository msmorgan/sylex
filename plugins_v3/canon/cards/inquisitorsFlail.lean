/-- Inquisitor's Flail -/
card inquisitorsFlail : Spelled :=
  { name := "Inquisitor's Flail", cost := [generic 2], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (replace (would (equipped creature) (dealDamage anyAmount (kind := some .combat)))
          (instead (instruct it (dealDamage (times 2 thatMuch))))),
        static
          (replace
            (would (a (and [isCreature, isOther]))
              (dealDamage anyAmount (equipped creature) (some .combat)))
            (instead (instruct it (dealDamage (times 2 thatMuch) (equipped creature))))),
        equip [generic 2] ] }
