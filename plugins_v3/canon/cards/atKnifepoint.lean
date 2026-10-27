card atKnifepoint : Spelled :=
  { name := "At Knifepoint", cost := [generic 1, pip .black, pip .red],
    types := [.enchantment], text := [(static (partScope .turn you
                  (abilityGrant (allOf (and [isOutlaw, youControl])) firstStrike))), (whenever (does
                    you commitCrime) (create 1 (written
                          { characteristics :=
                            { colors := [.red], types := [.creature], subtypes := [mercenary],
                              text :=
                                [ activatedOnlyDuring tapSymbol
                                    (get (target (and [isCreature, youControl])) (up 1) (up 0)
                                      untilEndOfTurn)
                                    asSorcery ],
                              power := stat 1, toughness := stat 1 } })) (limit := oncePerTurn))] }
