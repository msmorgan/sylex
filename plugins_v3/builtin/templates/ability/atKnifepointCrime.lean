template atKnifepointCrime : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := whenever (does you commitCrime) (create 1 (written
      { characteristics :=
        { colors := [.red], types := [.creature], subtypes := [mercenary],
          text :=
            [ activatedOnlyDuring tapSymbol
                (get (target (and [isCreature, youControl])) (up 1) (up 0)
                  untilEndOfTurn)
                asSorcery ],
          power := stat 1, toughness := stat 1 } })) (limit := oncePerTurn)
}
