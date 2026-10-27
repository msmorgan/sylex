/-- Tithe Taker -/
card titheTaker : Spelled :=
  { name := "Tithe Taker", cost := [generic 1, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ static (partScope .turn you
          (conjunction [ spellsCost (and [isSpell, did yourOpponents (cast which) thisObject])
              (more 1),
              costShift (allOf (and [ abilityHead .anyActivated,
                  did yourOpponents (activate which) thisObject,
                                    not isManaAbility ]))
                (more 1) ])),
        afterlife 1 ],
    power := stat 2, toughness := stat 1 }
