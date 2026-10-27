/-- Arcane Proxy -/
card arcaneProxy : Spelled := { body := .prototype
  { characteristics :=
    { name := "Arcane Proxy", cost := [generic 7], types := [.artifact, .creature],
      subtypes := [wizard],
      text :=
        [ prototype,
          when (does thisCreature enter) (if_ := is it (did you (cast which) thisObject))
            (sequentially
              [ exile
                  (target (and [ isInstantOrSorcery, isCard,
                                  hasManaValue .atMost (powerOf thisCreature),
                                  in_ yourGraveyard ])),
                copy .fromCardZone (that card) 1 [],
                may (cast (that .copy) (payment := withoutPaying)) ]) ],
      power := stat 4, toughness := stat 3 } }
  (prototypeAlt [generic 1, pip .blue, pip .blue] 2 1) }
