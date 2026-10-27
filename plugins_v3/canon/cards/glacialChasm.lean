card glacialChasm : Spelled :=
  { name := "Glacial Chasm", types := [.land],
    text :=
      [ cumulativeUpkeep (payLife 2),
        when (does thisLand enter) (sacrifice (a isLand)),
        static (cant (allOf (and [isCreature, youControl])) attack),
        static
          (replace (would any (dealDamage anyAmount you)) (instead (preventDamage allDamage))) ] }
